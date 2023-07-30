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
    Folder(Files),
    ListenCommandFlag,
    Stdin,
}

pub struct PathAndLineCount {
    path: String,
    line_count: usize,
}

pub struct Files {
    paths: Vec<String>,
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

    if let Some(file_or_folder) = args.file_path {
        return determine_input(file_or_folder, follow);
    }

    if is_stdin && args.file_path.is_none() && args.listen_command.is_none() {
        return Ok(Input::Stdin);
    }

    if args.listen_command.is_some() && !is_stdin && args.file_path.is_none() {
        return Ok(Input::ListenCommandFlag);
    }

    Err(Error {
        exit_code: 1,
        message: "Could not determine input".to_string(),
    })
}

fn determine_input(path: String, follow: bool) -> Result<Input, Error> {
    match check_path_type(&path)? {
        PathType::File => {
            let line_count = count_lines(&path, follow);
            Ok(Input::File(PathAndLineCount { path, line_count }))
        }
        PathType::Folder => {
            let paths = list_files_in_directory(Path::new(&path))?;
            Ok(Input::Folder(Files { paths }))
        }
    }
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
            exit_code: 1,
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

fn list_files_in_directory(path: &Path) -> Result<Vec<String>, Error> {
    let mut files = Vec::new();

    if path.is_dir() {
        for entry_result in fs::read_dir(path).map_err(|_| Error {
            exit_code: 1,
            message: "Unable to read directory".into(),
        })? {
            let entry = entry_result.map_err(|_| Error {
                exit_code: 1,
                message: "Unable to read directory entry".into(),
            })?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                files.push(
                    entry_path
                        .to_str()
                        .ok_or(Error {
                            exit_code: 1,
                            message: "Non-UTF8 filename".into(),
                        })?
                        .to_string(),
                );
            }
        }
    }

    Ok(files)
}
