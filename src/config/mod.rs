use crate::cli::Arguments;
use miette::Diagnostic;
use nu_ansi_term::Color::{Magenta, Yellow};
use std::cmp::PartialEq;
use std::fs;
use std::fs::File;
use std::io::{self, IsTerminal, Read, stdin};
use std::path::{Path, PathBuf};
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
    pub line_count: usize,
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
}

pub fn get_io_config(args: &Arguments) -> Result<InputOutputConfig, ConfigError> {
    let source = get_source(args)?;
    let target = get_target(args, &source);

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

fn get_target(args: &Arguments, input: &Source) -> Target {
    if let Some(var) = &args.pager {
        return Target::CustomPager(CustomPagerOptions { command: var.into() });
    }

    if *input == Source::Stdin || args.to_stdout {
        return Target::Stdout;
    }

    let follow_mode = if args.exec.is_some() { true } else { args.follow };

    Target::Less(LessOptions { follow: follow_mode })
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

    let line_count = count_lines(&path)?;

    Ok(Source::File(FileInfo {
        path,
        line_count,
        terminate_after_first_read,
    }))
}

fn count_lines<P: AsRef<Path>>(file_path: P) -> Result<usize, ConfigError> {
    let file = File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut count = 0;
    let mut buffer = [0; 8192]; // 8 KB buffer

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        count += buffer[..bytes_read].iter().filter(|&&c| c == b'\n').count();
    }

    Ok(count)
}
