use crate::types::{Error, GENERAL_ERROR};
use std::fs;
use std::fs::{DirEntry, File};
use std::io::BufRead;
use std::path::Path;

pub fn list_files_in_directory(path: &Path) -> Result<Vec<String>, Error> {
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

pub fn count_lines<P: AsRef<Path>>(file_path: P) -> usize {
    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    reader.lines().count()
}
