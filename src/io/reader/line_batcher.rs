use anyhow::Result;
use memchr::{memchr, memrchr};
use std::io::BufRead;

pub const BUF_READER_CAPACITY: usize = 64 * 1024;

#[derive(Debug)]
pub enum ReadResult {
    Eof,
    Batch(Vec<String>),
}

/// Returns the complete lines already buffered in `reader` as a `Batch`,
/// awaiting more input if the buffered line is partial.
pub fn read_lines<R>(reader: &mut R) -> Result<ReadResult>
where
    R: BufRead,
{
    let buf = reader.fill_buf()?;

    if buf.is_empty() {
        return Ok(ReadResult::Eof);
    }

    match memrchr(b'\n', buf) {
        Some(last) if memchr(b'\n', &buf[..last]).is_some() => {
            let lines = split_lines(&buf[..last]);
            reader.consume(last + 1);

            Ok(ReadResult::Batch(lines))
        }
        _ => read_line(reader),
    }
}

fn read_line<R>(reader: &mut R) -> Result<ReadResult>
where
    R: BufRead,
{
    let mut buf = Vec::new();
    reader.read_until(b'\n', &mut buf)?;

    Ok(ReadResult::Batch(vec![decode_line(&buf)]))
}

fn split_lines(buf: &[u8]) -> Vec<String> {
    buf.split(|&b| b == b'\n').map(decode_line).collect()
}

/// Strips a trailing `\n` or `\r\n` and lossy-decodes the bytes.
pub fn decode_line(bytes: &[u8]) -> String {
    let bytes = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    let bytes = bytes.strip_suffix(b"\r").unwrap_or(bytes);
    String::from_utf8_lossy(bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read};
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn test_read_complete_lines_multiple_lines() {
        let input = b"line 1\nline 2\nline 3\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_do_not_omit_empty_lines() {
        let input = b"line 1\n\nline 3\n\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["line 1", "", "line 3", ""]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_multiple_lines_no_final_newline() {
        let input = b"line 1\nline 2\nline 3";
        let mut reader = BufReader::new(&input[..]);
        let mut lines = vec![];

        let lines_first_read = read_lines(&mut reader).unwrap();
        let lines_second_read = read_lines(&mut reader).unwrap();

        if let ReadResult::Batch(batch) = lines_first_read {
            lines = [lines, batch].concat();
        }

        if let ReadResult::Batch(batch) = lines_second_read {
            lines.extend(batch);
        }

        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
    }

    #[test]
    fn test_read_eof_without_new_line_should_still_return_the_whole_string() {
        let input = b"incomplete line without newline";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["incomplete line without newline"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_single_complete_line() {
        let input = b"single complete line\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["single complete line"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_empty_input() {
        let input = b"";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Eof) = read_lines(&mut reader) {
            // success
        } else {
            panic!("Expected Ok(ReadResult::Eof), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_partial_line_pending() {
        let rx = read_lines_in_background(b"incomplete line without newline");

        let result = rx.recv_timeout(Duration::from_millis(100));

        assert!(result.is_err(), "Expected read_lines to block without newline or EOF");
    }

    #[test]
    fn test_read_available_lines() {
        let rx = read_lines_in_background(b"extract lines\n that are ready\n to be extracted\n but not more than that");

        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(ReadResult::Batch(lines))) => {
                assert_eq!(lines, vec!["extract lines", " that are ready", " to be extracted"]);
            }
            other => panic!("Expected Ok(ReadResult::Batch), got {other:?}"),
        }
    }

    /// Runs `read_lines` over a source that blocks forever once its initial
    /// data is consumed, so tests can assert on blocking behavior with
    /// timeouts. A still-blocked thread dies with the test process.
    fn read_lines_in_background(input: &[u8]) -> mpsc::Receiver<Result<ReadResult>> {
        let source = BlocksAfterInitialRead::new(input);
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let mut reader = BufReader::new(source);
            let _ = tx.send(read_lines(&mut reader));
        });

        rx
    }

    struct BlocksAfterInitialRead {
        data: Vec<u8>,
        position: usize,
    }

    impl BlocksAfterInitialRead {
        fn new(initial_data: &[u8]) -> Self {
            Self {
                data: initial_data.to_vec(),
                position: 0,
            }
        }
    }

    impl Read for BlocksAfterInitialRead {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.position < self.data.len() {
                let remaining = &self.data[self.position..];
                let to_read = remaining.len().min(buf.len());
                buf[..to_read].copy_from_slice(&remaining[..to_read]);
                self.position += to_read;
                Ok(to_read)
            } else {
                loop {
                    std::thread::park();
                }
            }
        }
    }

    #[test]
    fn test_crlf_batch() {
        let input = b"line1\r\nline2\r\nline3\r\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["line1", "line2", "line3"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_crlf_single_line() {
        let input = b"line1\r\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["line1"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_non_utf8_single_line() {
        let input = b"caf\xe9\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines.len(), 1);
            assert!(lines[0].starts_with("caf"));
            assert!(lines[0].contains('\u{FFFD}'));
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_mixed_line_endings() {
        let input = b"unix\nwindows\r\nunix2\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader) {
            assert_eq!(lines, vec!["unix", "windows", "unix2"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }
}
