use crate::cli::Cli;
use miette::Diagnostic;
use owo_colors::OwoColorize;
use std::fs::{DirEntry, File};
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

pub struct FolderInfo {
    pub folder_name: PathBuf,
    pub file_paths: Vec<PathBuf>,
}

pub enum Input {
    File(PathAndLineCount),
    Folder(FolderInfo),
    Command(String),
    Stdin,
}

pub enum Output {
    Less,
    CustomPager(String),
    Stdout,
    Suppress,
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
    #[error("Path is neither a file nor a directory")]
    PathNotFileNorDirectory,
    #[error("Path is not a directory")]
    PathNotDirectory,
    #[error("Unable to read directory")]
    UnableToReadDirectory,
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
}

pub fn create_config(args: &Cli) -> Result<Config, ConfigError> {
    let has_data_from_stdin = !stdin().is_terminal();

    validate_input(
        has_data_from_stdin,
        args.file_or_folder_path.is_some(),
        args.listen_command.is_some(),
    )?;

    let input = get_input(args, has_data_from_stdin)?;
    let output = get_output(has_data_from_stdin, args.to_stdout, args.suppress_output);
    let follow = should_follow(args.follow, args.listen_command.is_some(), &input);

    Ok(Config {
        input,
        output,
        follow,
        start_at_end: args.start_at_end,
    })
}

fn validate_input(
    has_data_from_stdin: bool,
    has_file_or_folder_input: bool,
    has_follow_command_input: bool,
) -> Result<(), ConfigError> {
    if !has_data_from_stdin && !has_file_or_folder_input && !has_follow_command_input {
        return Err(ConfigError::MissingFilename("tspin --help".magenta().to_string()));
    }

    if has_file_or_folder_input && has_follow_command_input {
        return Err(ConfigError::CannotReadBothFileAndListenCommand(
            "--listen-command".magenta().to_string(),
        ));
    }

    Ok(())
}

fn get_input(args: &Cli, has_data_from_stdin: bool) -> Result<Input, ConfigError> {
    if has_data_from_stdin {
        Ok(Input::Stdin)
    } else if let Some(command) = &args.listen_command {
        Ok(Input::Command(command.clone()))
    } else if let Some(path) = &args.file_or_folder_path {
        let path = PathBuf::from(path);
        process_path_input(path)
    } else {
        Err(ConfigError::CouldNotDetermineInputType)
    }
}

fn get_output(has_data_from_stdin: bool, to_stdout: bool, suppress_output: bool) -> Output {
    if suppress_output {
        Output::Suppress
    } else if let Ok(var) = env::var("TAILSPIN_PAGER") {
        Output::CustomPager(var)
    } else if has_data_from_stdin || to_stdout {
        Output::Stdout
    } else {
        Output::Less
    }
}

fn process_path_input(path: PathBuf) -> Result<Input, ConfigError> {
    match get_path_type(&path)? {
        PathType::File => {
            let line_count = count_lines(&path)?;
            Ok(Input::File(PathAndLineCount { path, line_count }))
        }
        PathType::Folder => {
            let mut file_paths = list_files_in_directory(&path)?;
            file_paths.sort();
            Ok(Input::Folder(FolderInfo {
                folder_name: path,
                file_paths,
            }))
        }
    }
}

enum PathType {
    File,
    Folder,
}

fn get_path_type(path: &Path) -> Result<PathType, ConfigError> {
    let metadata = fs::metadata(path).map_err(|_| ConfigError::NoSuchFileOrDirectory(path.display().to_string()))?;
    if metadata.is_file() {
        Ok(PathType::File)
    } else if metadata.is_dir() {
        Ok(PathType::Folder)
    } else {
        Err(ConfigError::PathNotFileNorDirectory)
    }
}

const fn should_follow(follow_flag: bool, has_command: bool, input: &Input) -> bool {
    if has_command || matches!(input, Input::Folder(_)) {
        true
    } else {
        follow_flag
    }
}

fn list_files_in_directory(path: &Path) -> Result<Vec<PathBuf>, ConfigError> {
    if !path.is_dir() {
        return Err(ConfigError::PathNotDirectory);
    }

    fs::read_dir(path)
        .map_err(|_| ConfigError::UnableToReadDirectory)?
        .filter_map(Result::ok)
        .filter(is_normal_file)
        .map(|entry| Ok(entry.path()))
        .collect()
}

fn is_normal_file(entry: &DirEntry) -> bool {
    entry.path().is_file()
        && entry
            .path()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| !name.starts_with('.'))
            .unwrap_or(false)
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
