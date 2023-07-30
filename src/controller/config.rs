use crate::cli::Cli;
use std::fs;
use std::fs::File;
use std::io::{stdin, BufRead, IsTerminal};
use std::path::Path;

pub struct Error {
    exit_code: usize,
    message: String,
}

pub struct Config {
    input: Input,
    output: Output,
    follow: bool,
}

pub enum Input {
    File(PathAndLineCount),
    Folder(String),
    ListenCommandFlag,
    Stdin,
}

pub struct PathAndLineCount {
    path: String,
    line_count: usize,
}

pub enum Output {
    TempFile,
    Stdout,
}

enum PathType {
    File,
    Folder,
}

pub fn create_config(args: Cli) -> Result<Config, Error> {
    let follow = should_follow(args.follow, args.listen_command.is_some());
    let input = get_input(args, follow)?;

    let config = Config {
        input,
        output: Output::Stdout,
        follow,
    };

    Ok(config)
}

fn get_input(args: Cli, follow: bool) -> Result<Input, Error> {
    let is_stdin = !stdin().is_terminal();

    if !is_stdin && args.listen_command.is_none() {
        return Err(Error {
            exit_code: 0,
            message: "Missing filename (`spin --help` for help) ".to_string(),
        });
    }

    if is_stdin && args.file_path.is_some() {
        return Err(Error {
            exit_code: 1,
            message: "Ambigous input: both ".to_string(),
        });
    }

    if let Some(path) = args.file_path {
        let path_type = check_path_type(path.clone())?;

        return match path_type {
            PathType::File => {
                let line_count = count_lines(path.clone(), follow);
                let path_and_line_count = PathAndLineCount { path, line_count };

                Ok(Input::File(path_and_line_count))
            }
            PathType::Folder => Ok(Input::Folder(path)),
        };
    }

    if is_stdin && args.file_path.is_none() && args.listen_command.is_none() {
        return Ok(Input::Stdin);
    }

    Err(Error {
        exit_code: 1,
        message: "Could not determine input".to_string(),
    })
}

fn check_path_type<P: AsRef<Path>>(path: P) -> Result<PathType, Error> {
    let metadata = fs::metadata(path.as_ref()).map_err(|_| Error {
        exit_code: 1,
        message: "Failed to access path metadata".into(),
    })?;

    if metadata.is_file() {
        Ok(PathType::File)
    } else if metadata.is_dir() {
        Ok(PathType::Folder)
    } else {
        Err(Error {
            exit_code: 2,
            message: "Path is neither a file nor a directory".into(),
        })
    }
}

fn should_follow(follow: bool, has_follow_command: bool) -> bool {
    if has_follow_command {
        return true;
    }

    follow
}

fn count_lines<P: AsRef<Path>>(file_path: P, follow: bool) -> usize {
    if follow {
        return 1;
    }

    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    reader.lines().count()
}
