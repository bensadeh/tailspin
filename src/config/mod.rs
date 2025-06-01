use crate::cli::Arguments;
use miette::Diagnostic;
use nu_ansi_term::Color::{Magenta, Yellow};
use std::cmp::PartialEq;
use std::fs;
use std::io::{self, IsTerminal, stdin};
use std::path::PathBuf;
use thiserror::Error;

pub struct InputOutputConfig {
    pub source: Source,
    pub target: Target,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Source {
    File(FileInfo),
    Command(String),
    Stdin,
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub struct FileInfo {
    pub path: PathBuf,
    pub terminate_after_first_read: bool,
}

pub enum Target {
    Less(LessOptions),
    CustomPager(CustomPagerOptions),
    Stdout,
}

pub struct LessOptions {
    pub follow: bool,
}

pub struct CustomPagerOptions {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ConfigError {
    #[error("Missing filename ({} for help)", Magenta.paint("tspin --help").to_string())]
    #[diagnostic(severity(Warning))]
    MissingFilename,

    #[error("Cannot read from both file and {}", Magenta.paint("--listen-command").to_string())]
    CannotReadBothFileAndListenCommand,

    #[error("Could not determine input type")]
    CouldNotDetermineInputType,

    #[error("{0}: No such file or directory")]
    #[diagnostic(severity(Warning))]
    NoSuchFileOrDirectory(String),

    #[error("Path is not a file")]
    PathNotFile,

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    #[error("Could not parse custom pager command")]
    CouldNotParseCustomPagerCommand,
}

pub fn get_io_config(args: &Arguments) -> Result<InputOutputConfig, ConfigError> {
    let source = get_source(args)?;
    let target = get_target(args, &source)?;

    Ok(InputOutputConfig { source, target })
}

fn get_source(args: &Arguments) -> Result<Source, ConfigError> {
    let std_in_has_data = !stdin().is_terminal();

    if args.file_path.is_none() && !std_in_has_data && args.exec.is_none() {
        return Err(ConfigError::MissingFilename);
    }

    if args.file_path.is_some() && args.exec.is_some() {
        return Err(ConfigError::CannotReadBothFileAndListenCommand);
    }

    if let Some(path) = &args.file_path {
        let terminate_after_first_read = args.to_stdout && !args.follow;
        return process_path_input(path.into(), terminate_after_first_read);
    }

    if std_in_has_data {
        return Ok(Source::Stdin);
    }

    if let Some(command) = &args.exec {
        return Ok(Source::Command(command.clone()));
    }

    Err(ConfigError::CouldNotDetermineInputType)
}

fn get_target(args: &Arguments, input: &Source) -> Result<Target, ConfigError> {
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

fn split_custom_pager_command(raw_command: &str) -> Result<CustomPagerOptions, ConfigError> {
    let raw_args = if cfg!(windows) {
        winsplit::split(raw_command)
    } else {
        shell_words::split(raw_command).unwrap_or_default()
    };

    let (command, args) = match raw_args.split_first() {
        Some((first, rest)) if !rest.is_empty() => (first.to_string(), rest.to_vec()),
        Some(_) => return Err(ConfigError::CouldNotParseCustomPagerCommand), // Command without args
        None => return Err(ConfigError::CouldNotParseCustomPagerCommand),    // Empty args
    };

    Ok(CustomPagerOptions { command, args })
}

fn process_path_input(path: PathBuf, terminate_after_first_read: bool) -> Result<Source, ConfigError> {
    if !path.exists() {
        let path_display = path.display().to_string();
        let path_colored = Yellow.paint(path_display).to_string();

        return Err(ConfigError::NoSuchFileOrDirectory(path_colored));
    }

    if !fs::metadata(&path)?.is_file() {
        return Err(ConfigError::PathNotFile);
    }

    Ok(Source::File(FileInfo {
        path,
        terminate_after_first_read,
    }))
}
