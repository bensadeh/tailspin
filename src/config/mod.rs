use crate::cli::Cli;
use color_eyre::owo_colors::OwoColorize;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{self, stdin, IsTerminal, Read};
use std::path::{Path, PathBuf};
use thiserror::Error;

enum InputType {
    Stdin,
    Command(String),
    FileOrFolder(PathBuf),
}

enum PathType {
    File,
    Folder,
}

pub fn create_config(args: &Cli) -> Result<Config, ConfigError> {
    let has_data_from_stdin = !stdin().is_terminal();

    validate_input(
        has_data_from_stdin,
        args.file_or_folder_path.is_some(),
        args.listen_command.is_some(),
    )?;

    let input_type = determine_input_type(args, has_data_from_stdin)?;
    let input = get_input(input_type)?;
    let output = get_output(has_data_from_stdin, args.to_stdout, args.suppress_output);
    let follow = should_follow(args.follow, args.listen_command.is_some(), &input);

    let config = Config {
        input,
        output,
        follow,
        start_at_end: args.start_at_end,
    };

    Ok(config)
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

fn determine_input_type(args: &Cli, has_data_from_stdin: bool) -> Result<InputType, ConfigError> {
    if has_data_from_stdin {
        Ok(InputType::Stdin)
    } else if let Some(command) = &args.listen_command {
        Ok(InputType::Command(command.clone()))
    } else if let Some(path) = &args.file_or_folder_path {
        Ok(InputType::FileOrFolder(PathBuf::from(path)))
    } else {
        Err(ConfigError::CouldNotDetermineInputType)
    }
}

fn get_input(input_type: InputType) -> Result<Input, ConfigError> {
    match input_type {
        InputType::Stdin => Ok(Input::Stdin),
        InputType::Command(cmd) => Ok(Input::Command(cmd)),
        InputType::FileOrFolder(path) => determine_input(path),
    }
}

const fn get_output(has_data_from_stdin: bool, is_print_flag: bool, suppress_output: bool) -> Output {
    if suppress_output {
        Output::Suppress
    } else if has_data_from_stdin || is_print_flag {
        Output::Stdout
    } else {
        Output::TempFile
    }
}

fn determine_input(path: PathBuf) -> Result<Input, ConfigError> {
    match check_path_type(&path)? {
        PathType::File => {
            let line_count = count_lines(&path)?;
            Ok(Input::File(PathAndLineCount { path, line_count }))
        }
        PathType::Folder => {
            let mut paths = list_files_in_directory(&path)?;
            paths.sort();

            Ok(Input::Folder(FolderInfo {
                folder_name: path,
                file_paths: paths,
            }))
        }
    }
}

fn check_path_type(path: &Path) -> Result<PathType, ConfigError> {
    let metadata = fs::metadata(path).map_err(|_| ConfigError::NoSuchFileOrDirectory(path.display().to_string()))?;

    if metadata.is_file() {
        Ok(PathType::File)
    } else if metadata.is_dir() {
        Ok(PathType::Folder)
    } else {
        Err(ConfigError::PathNotFileNorDirectory)
    }
}

const fn should_follow(follow: bool, has_follow_command: bool, input: &Input) -> bool {
    if has_follow_command || matches!(input, Input::Folder(_)) {
        true
    } else {
        follow
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

#[derive(Debug, Error)]
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
    TempFile,
    Stdout,
    Suppress,
}
