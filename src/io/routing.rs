use nu_ansi_term::Color::{Magenta, Yellow};
use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Source {
    File(FileInfo),
    Command(String),
    Stdin,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
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

    let (command, args) = match raw_args.split_first() {
        Some((first, rest)) if !rest.is_empty() => (first.clone(), rest.to_vec()),
        Some(_) | None => return Err(RoutingError::CouldNotParseCustomPagerCommand),
    };

    Ok(CustomPagerOptions { command, args })
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
