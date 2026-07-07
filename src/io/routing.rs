use nu_ansi_term::Color::{Magenta, Yellow};
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Source {
    File(FileInfo),
    Command(String),
    Stdin,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub terminate_after_first_read: bool,
}

#[derive(Debug)]
pub enum Target {
    Less(LessOptions),
    CustomPager(CustomPagerOptions),
    Stdout,
}

#[derive(Debug)]
pub struct LessOptions {
    pub follow: bool,
}

#[derive(Debug)]
pub struct CustomPagerOptions {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Error)]
pub enum RoutingError {
    #[error("Cannot read from both file and {}", Magenta.paint("--exec").to_string())]
    CannotReadBothFileAndExec,

    #[error("Could not determine input type")]
    CouldNotDetermineInputType,

    #[error("{0}: No such file or directory")]
    NoSuchFileOrDirectory(String),

    #[error("Path is not a file")]
    PathNotFile,

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    #[error("Could not parse custom pager command")]
    CouldNotParseCustomPagerCommand,

    #[error("Custom pager command must include the {} placeholder", Magenta.paint("[FILE]").to_string())]
    MissingFilePlaceholder,
}

#[derive(Debug)]
pub struct IoArgs {
    pub file_path: Option<PathBuf>,
    pub exec: Option<String>,
    pub to_stdout: bool,
    pub follow: bool,
    pub pager: Option<String>,
    pub std_in_has_data: bool,
}

pub fn resolve(args: IoArgs) -> Result<(Source, Target), RoutingError> {
    let source = get_source(&args)?;
    let target = get_target(&args, &source)?;

    Ok((source, target))
}

fn get_source(args: &IoArgs) -> Result<Source, RoutingError> {
    if args.file_path.is_some() && args.exec.is_some() {
        return Err(RoutingError::CannotReadBothFileAndExec);
    }

    if let Some(path) = &args.file_path {
        // Print mode without --follow is the only case that ends at EOF: with
        // a pager the reader keeps following so the backing temp file picks
        // up new lines the pager can reveal (reload or press F in less).
        let terminate_after_first_read = args.to_stdout && !args.follow;
        return process_path_input(path.into(), terminate_after_first_read);
    }

    if let Some(command) = &args.exec {
        return Ok(Source::Command(command.clone()));
    }

    if args.std_in_has_data {
        return Ok(Source::Stdin);
    }

    Err(RoutingError::CouldNotDetermineInputType)
}

fn get_target(args: &IoArgs, input: &Source) -> Result<Target, RoutingError> {
    if *input == Source::Stdin || args.to_stdout {
        return Ok(Target::Stdout);
    }

    if let Some(command) = &args.pager {
        let custom_pager_options = split_custom_pager_command(command)?;

        return Ok(Target::CustomPager(custom_pager_options));
    }

    let follow_mode = if args.exec.is_some() { true } else { args.follow };

    Ok(Target::Less(LessOptions { follow: follow_mode }))
}

fn split_custom_pager_command(raw_command: &str) -> Result<CustomPagerOptions, RoutingError> {
    let raw_args = shell_words::split(raw_command).unwrap_or_default();

    let Some((command, args)) = raw_args.split_first() else {
        return Err(RoutingError::CouldNotParseCustomPagerCommand);
    };

    // The pager only receives the temp file through the [FILE] placeholder
    // (see `io::presenter::pager`), so a command without one would spawn
    // with nothing to page.
    if !args.iter().any(|arg| arg.contains("[FILE]")) {
        return Err(RoutingError::MissingFilePlaceholder);
    }

    Ok(CustomPagerOptions {
        command: command.clone(),
        args: args.to_vec(),
    })
}

fn process_path_input(path: PathBuf, terminate_after_first_read: bool) -> Result<Source, RoutingError> {
    if !path.exists() {
        let path_display = path.display().to_string();
        let path_colored = Yellow.paint(path_display).to_string();

        return Err(RoutingError::NoSuchFileOrDirectory(path_colored));
    }

    if !fs::metadata(&path)?.is_file() {
        return Err(RoutingError::PathNotFile);
    }

    Ok(Source::File(FileInfo {
        path,
        terminate_after_first_read,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pager_command_with_placeholder_splits_into_command_and_args() {
        let options = split_custom_pager_command("ov -f [FILE]").unwrap();

        assert_eq!(options.command, "ov");
        assert_eq!(options.args, vec!["-f", "[FILE]"]);
    }

    #[test]
    fn pager_command_without_placeholder_is_rejected() {
        let err = split_custom_pager_command("ov -f").unwrap_err();
        assert!(matches!(err, RoutingError::MissingFilePlaceholder));
    }

    #[test]
    fn bare_pager_command_is_rejected() {
        let err = split_custom_pager_command("ov").unwrap_err();
        assert!(matches!(err, RoutingError::MissingFilePlaceholder));
    }

    #[test]
    fn unparseable_pager_command_is_rejected() {
        for raw in ["", "ov 'unclosed [FILE]"] {
            let err = split_custom_pager_command(raw).unwrap_err();
            assert!(matches!(err, RoutingError::CouldNotParseCustomPagerCommand));
        }
    }
}
