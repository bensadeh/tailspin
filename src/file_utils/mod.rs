use crate::types::{Error, GENERAL_ERROR};
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub fn list_files_in_directory(path: &Path) -> Result<Vec<String>, Error> {
    let mut files = Vec::new();

    if path.is_dir() {
        for entry_result in fs::read_dir(path).map_err(|_| Error {
            exit_code: GENERAL_ERROR,
            message: "Unable to read directory".into(),
        })? {
            let entry = entry_result.map_err(|_| Error {
                exit_code: GENERAL_ERROR,
                message: "Unable to read directory entry".into(),
            })?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                files.push(
                    entry_path
                        .to_str()
                        .ok_or(Error {
                            exit_code: GENERAL_ERROR,
                            message: "Non-UTF8 filename".into(),
                        })?
                        .to_string(),
                );
            }
        }
    }

    Ok(files)
}

pub fn count_lines<P: AsRef<Path>>(file_path: P) -> usize {
    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    reader.lines().count()
}
