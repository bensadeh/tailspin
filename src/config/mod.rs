use crate::cli::Cli;
use crate::highlighter::config::CliOpts;
use crate::types::{
    Config, Error, FolderInfo, Input, Output, PathAndLineCount, GENERAL_ERROR, MISUSE_SHELL_BUILTIN, OK,
};
use color_eyre::owo_colors::OwoColorize;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{stdin, BufRead, IsTerminal};
use std::path::Path;
use std::process::exit;

enum InputType {
    Stdin,
    Command(String),
    FileOrFolder(String),
}

enum PathType {
    File,
    Folder,
}

pub fn create_config_or_exit_early(args: &Cli) -> Config {
    match create_config(args) {
        Ok(c) => c,
        Err(e) => {
            match e.exit_code {
                OK => println!("{}", e.message),
                _ => eprintln!("{}", e.message),
            }
            exit(e.exit_code);
        }
    }
}

fn create_config(args: &Cli) -> Result<Config, Error> {
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
) -> Result<(), Error> {
    if !has_data_from_stdin && !has_file_or_folder_input && !has_follow_command_input {
        return Err(Error {
            exit_code: OK,
            message: format!("Missing filename ({} for help)", "tspin --help".magenta()),
        });
    }

    if has_file_or_folder_input && has_follow_command_input {
        return Err(Error {
            exit_code: MISUSE_SHELL_BUILTIN,
            message: format!("Cannot read from both file and {}", "--listen-command".magenta()),
        });
    }

    Ok(())
}

fn determine_input_type(args: &Cli, has_data_from_stdin: bool) -> Result<InputType, Error> {
    if has_data_from_stdin {
        return Ok(InputType::Stdin);
    }

    if let Some(command) = &args.listen_command {
        return Ok(InputType::Command(command.clone()));
    }

    if let Some(path) = &args.file_or_folder_path {
        return Ok(InputType::FileOrFolder(path.clone()));
    }

    Err(Error {
        exit_code: GENERAL_ERROR,
        message: "Could not determine input type".to_string(),
    })
}

fn get_input(input_type: InputType) -> Result<Input, Error> {
    match input_type {
        InputType::Stdin => Ok(Input::Stdin),
        InputType::Command(cmd) => Ok(Input::Command(cmd)),
        InputType::FileOrFolder(path) => determine_input(path),
    }
}

const fn get_output(has_data_from_stdin: bool, is_print_flag: bool, suppress_output: bool) -> Output {
    if suppress_output {
        return Output::Suppress;
    }

    if has_data_from_stdin || is_print_flag {
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
            let mut paths = list_files_in_directory(Path::new(&path))?;
            paths.sort();

            Ok(Input::Folder(FolderInfo {
                folder_name: path,
                file_paths: paths,
            }))
        }
    }
}

fn check_path_type<P: AsRef<Path>>(path: P) -> Result<PathType, Error> {
    let metadata = fs::metadata(path.as_ref()).map_err(|_| Error {
        exit_code: GENERAL_ERROR,
        message: format!("{}: No such file or directory", path.as_ref().display().red()),
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

const fn should_follow(follow: bool, has_follow_command: bool, input: &Input) -> bool {
    if has_follow_command {
        return true;
    }

    if matches!(input, Input::Folder(_)) {
        return true;
    }

    follow
}

fn list_files_in_directory(path: &Path) -> Result<Vec<String>, Error> {
    if !path.is_dir() {
        return Err(Error {
            exit_code: GENERAL_ERROR,
            message: "Path is not a directory".into(),
        });
    }

    fs::read_dir(path)
        .map_err(|_| Error {
            exit_code: GENERAL_ERROR,
            message: "Unable to read directory".into(),
        })?
        .filter_map(Result::ok)
        .filter(is_normal_file)
        .map(entry_to_string)
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

fn entry_to_string(entry: DirEntry) -> Result<String, Error> {
    entry
        .path()
        .to_str()
        .ok_or(Error {
            exit_code: GENERAL_ERROR,
            message: "Non-UTF8 filename".into(),
        })
        .map(|s| s.to_string())
}

fn count_lines<P: AsRef<Path>>(file_path: P) -> usize {
    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    reader.lines().count()
}

pub const fn get_cli_opts_for_highlight_groups(cli: &Cli) -> CliOpts {
    CliOpts {
        enable_numbers: cli.enable_numbers,
        disable_numbers: cli.disable_numbers,
        enable_uuids: cli.enable_uuids,
        disable_uuids: cli.disable_uuids,
        enable_quotes: cli.enable_quotes,
        disable_quotes: cli.disable_quotes,
        enable_ip_addresses: cli.enable_ip_addresses,
        disable_ip_addresses: cli.disable_ip_addresses,
        enable_dates: cli.enable_dates,
        disable_dates: cli.disable_dates,
        enable_paths: cli.enable_paths,
        disable_paths: cli.disable_paths,
        enable_urls: cli.enable_urls,
        disable_urls: cli.disable_urls,
        enable_pointers: cli.enable_pointers,
        disable_pointers: cli.disable_pointers,
        enable_processes: cli.enable_processes,
        disable_processes: cli.disable_processes,
        enable_key_value_pairs: cli.enable_key_value_pairs,
        disable_key_value_pairs: cli.disable_key_value_pairs,
    }
}
