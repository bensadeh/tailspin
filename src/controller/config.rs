use crate::cli::Cli;
use crate::file_utils::{count_lines, list_files_in_directory};
use crate::types::{
    Config, Error, Files, Input, Output, PathAndLineCount, GENERAL_ERROR, MISUSE_SHELL_BUILTIN,
};
use std::fs;
use std::io::{stdin, IsTerminal};
use std::path::Path;

enum PathType {
    File,
    Folder,
}

pub fn create_config(args: Cli) -> Result<Config, Error> {
    let follow = should_follow(args.follow, args.listen_command.is_some());
    let is_stdin = !stdin().is_terminal();

    let input = get_input(args.file_path, args.listen_command, is_stdin)?;
    let output = get_output(is_stdin, args.to_stdout);

    let config = Config {
        input,
        output,
        follow,
    };

    Ok(config)
}

fn get_input(
    file_path: Option<String>,
    listen_command: Option<String>,
    is_stdin: bool,
) -> Result<Input, Error> {
    if !is_stdin && file_path.is_none() && listen_command.is_none() {
        return Err(Error {
            exit_code: GENERAL_ERROR,
            message: "Missing filename (`spin --help` for help) ".to_string(),
        });
    }

    if is_stdin && file_path.is_some() {
        return Err(Error {
            exit_code: MISUSE_SHELL_BUILTIN,
            message: "Cannot read from both stdin and --listen-command ".to_string(),
        });
    }

    if let Some(file_or_folder) = file_path {
        return determine_input(file_or_folder);
    }

    if is_stdin && file_path.is_none() && listen_command.is_none() {
        return Ok(Input::Stdin);
    }

    if let Some(command) = listen_command {
        return Ok(Input::Command(command));
    }

    Err(Error {
        exit_code: 1,
        message: "Could not determine input".to_string(),
    })
}

fn get_output(is_stdin: bool, to_stdout: bool) -> Output {
    if is_stdin || to_stdout {
        return Output::Stdout;
    }

    Output::TempFile
}

fn determine_input(path: String) -> Result<Input, Error> {
    match check_path_type(&path)? {
        PathType::File => {
            let line_count = count_lines(&path);
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
        exit_code: GENERAL_ERROR,
        message: "Failed to access path metadata".into(),
    })?;

    if metadata.is_file() {
        Ok(PathType::File)
    } else if metadata.is_dir() {
        Ok(PathType::Folder)
    } else {
        Err(Error {
            exit_code: GENERAL_ERROR,
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
