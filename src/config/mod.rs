use crate::cli::Arguments;
use miette::Diagnostic;
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::{self, stdin, IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::{env, fs};
use thiserror::Error;

pub struct InputOutputConfig {
    pub input: Input,
    pub output: Output,
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
    #[error("Missing filename ({} for help)", "tspin --help".magenta().to_string())]
    MissingFilename,

    #[error("Cannot read from both file and {}", "--listen-command".magenta().to_string())]
    CannotReadBothFileAndListenCommand,

    #[error("Could not determine input type")]
    CouldNotDetermineInputType,

    #[error("{0}: No such file or directory")]
    NoSuchFileOrDirectory(String),

    #[error("Path is not a file")]
    PathNotFile,

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
}

pub fn get_io_config(args: &Arguments) -> Result<InputOutputConfig, ConfigError> {
    let has_data_from_stdin = !stdin().is_terminal();

    validate_input(
        has_data_from_stdin,
        args.file_path.is_some(),
        args.listen_command.is_some(),
    )?;

    let input = get_input(args, has_data_from_stdin)?;
    let output = get_output(
        has_data_from_stdin,
        args.to_stdout,
        args.follow,
        args.listen_command.is_some(),
    );

    Ok(InputOutputConfig { input, output })
}

const fn validate_input(
    has_data_from_stdin: bool,
    has_file_input: bool,
    has_follow_command_input: bool,
) -> Result<(), ConfigError> {
    if !has_data_from_stdin && !has_file_input && !has_follow_command_input {
        return Err(ConfigError::MissingFilename);
    }

    if has_file_input && has_follow_command_input {
        return Err(ConfigError::CannotReadBothFileAndListenCommand);
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

fn get_output(has_data_from_stdin: bool, to_stdout: bool, follow: bool, has_listen_command: bool) -> Output {
    if let Ok(var) = env::var("TAILSPIN_PAGER") {
        Output::CustomPager(CustomPagerOptions { command: var })
    } else if has_data_from_stdin || to_stdout {
        Output::Stdout
    } else {
        let follow = if has_listen_command { true } else { follow };

        Output::Less(LessOptions { follow })
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
