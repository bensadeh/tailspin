use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use memchr::memchr_iter;
use miette::{Context, IntoDiagnostic, Result};

pub const BUFF_CAPACITY: usize = 64 * 1024;

pub fn count_lines<P: AsRef<Path>>(file_path: P) -> Result<usize> {
    let file = File::open(&file_path)
        .into_diagnostic()
        .wrap_err_with(|| format!("Could not open file: {:?}", file_path.as_ref()))?;

    let mut reader = BufReader::new(file);
    let mut count = 0usize;

    let mut buffer = [0u8; BUFF_CAPACITY];
    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .into_diagnostic()
            .wrap_err_with(|| format!("Error reading file: {:?}", file_path.as_ref()))?;

        if bytes_read == 0 {
            // EOF reached
            break;
        }

        count += memchr_iter(b'\n', &buffer[..bytes_read]).count();
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_count_lines_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test-file.txt");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello").unwrap();
        writeln!(file, "World").unwrap();
        writeln!(file, "Rust!").unwrap();

        let result = count_lines(&file_path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_count_lines_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");

        File::create(&file_path).unwrap();

        let result = count_lines(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_count_lines_no_newline() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("no-newline.txt");

        let mut file = File::create(&file_path).unwrap();
        write!(file, "No newline at the end").unwrap();

        let result = count_lines(&file_path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0, "Should have 0 newlines");
    }

    #[test]
    fn test_count_lines_large_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large.txt");

        let mut file = File::create(&file_path).unwrap();
        for i in 0..10000 {
            writeln!(file, "Line {}", i).unwrap();
        }

        let result = count_lines(&file_path);
        assert!(result.is_ok());

        assert_eq!(result.unwrap(), 10000);
    }
}
