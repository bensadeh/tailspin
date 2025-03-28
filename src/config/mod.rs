use crate::cli::Arguments;
use miette::Diagnostic;
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::{self, stdin, IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::{env, fs};
use thiserror::Error;

pub struct Config {
    pub input: Input,
    pub output: Output,
    pub follow: bool,
    pub start_at_end: bool,
}

pub struct PathAndLineCount {
    pub path: PathBuf,
    pub line_count: usize,
}

pub enum Input {
    File(PathAndLineCount),
    Command(String),
    Stdin,
}

pub enum Output {
    Less,
    CustomPager(String),
    Stdout,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ConfigError {
    #[error("Missing filename ({0} for help)")]
    MissingFilename(String),

    #[error("Cannot read from both file and {0}")]
    CannotReadBothFileAndListenCommand(String),

    #[error("Could not determine input type")]
    CouldNotDetermineInputType,

    #[error("{0}: No such file or directory")]
    NoSuchFileOrDirectory(String),

    #[error("Path is not a file")]
    PathNotFile,

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
}

pub fn create_config(args: &Arguments) -> Result<Config, ConfigError> {
    let has_data_from_stdin = !stdin().is_terminal();

    validate_input(
        has_data_from_stdin,
        args.file_path.is_some(),
        args.listen_command.is_some(),
    )?;

    let input = get_input(args, has_data_from_stdin)?;
    let output = get_output(has_data_from_stdin, args.to_stdout);
    let follow = should_follow(args.follow, args.listen_command.is_some());

    Ok(Config {
        input,
        output,
        follow,
        start_at_end: args.start_at_end,
    })
}

fn validate_input(
    has_data_from_stdin: bool,
    has_file_input: bool,
    has_follow_command_input: bool,
) -> Result<(), ConfigError> {
    if !has_data_from_stdin && !has_file_input && !has_follow_command_input {
        return Err(ConfigError::MissingFilename("tspin --help".magenta().to_string()));
    }

    if has_file_input && has_follow_command_input {
        return Err(ConfigError::CannotReadBothFileAndListenCommand(
            "--listen-command".magenta().to_string(),
        ));
    }

    Ok(())
}

fn get_input(args: &Arguments, has_data_from_stdin: bool) -> Result<Input, ConfigError> {
    if has_data_from_stdin {
        Ok(Input::Stdin)
    } else if let Some(command) = &args.listen_command {
        Ok(Input::Command(command.clone()))
    } else if let Some(path_str) = &args.file_path {
        let path = PathBuf::from(path_str);
        process_path_input(path)
    } else {
        Err(ConfigError::CouldNotDetermineInputType)
    }
}

fn get_output(has_data_from_stdin: bool, to_stdout: bool) -> Output {
    if let Ok(var) = env::var("TAILSPIN_PAGER") {
        Output::CustomPager(var)
    } else if has_data_from_stdin || to_stdout {
        Output::Stdout
    } else {
        Output::Less
    }
}

fn process_path_input(path: PathBuf) -> Result<Input, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::NoSuchFileOrDirectory(path.display().to_string()));
    }

    let metadata = fs::metadata(&path)?;
    if !metadata.is_file() {
        return Err(ConfigError::PathNotFile);
    }

    let line_count = count_lines(&path)?;

    Ok(Input::File(PathAndLineCount { path, line_count }))
}

const fn should_follow(follow_flag: bool, has_command: bool) -> bool {
    if has_command { true } else { follow_flag }
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
