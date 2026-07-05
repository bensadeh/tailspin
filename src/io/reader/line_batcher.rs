use anyhow::Result;
use memchr::{memchr_iter, memrchr};
#[cfg(test)]
use std::borrow::Cow;
use std::io::BufRead;
use std::ops::Range;

pub const BUF_READER_CAPACITY: usize = 1024 * 1024;

/// A batch of complete lines sharing one buffer. `lines` holds each line's
/// byte range within `buf`, with line endings (`\n`, `\r\n`) stripped.
#[derive(Debug)]
pub struct LineBatch {
    pub buf: Vec<u8>,
    pub lines: Vec<Range<usize>>,
}

impl LineBatch {
    /// Splits a chunk with no trailing newline into a batch of lines.
    fn from_chunk(chunk: &[u8]) -> Self {
        let buf = chunk.to_vec();
        let mut lines = Vec::new();
        let mut start = 0;
        for newline in memchr_iter(b'\n', &buf) {
            lines.push(strip_cr(&buf, start..newline));
            start = newline + 1;
        }
        lines.push(strip_cr(&buf, start..buf.len()));

        Self { buf, lines }
    }

    /// A batch holding a single line; strips one trailing `\n` or `\r\n`.
    pub fn single_line(line: &[u8]) -> Self {
        let line = line.strip_suffix(b"\n").unwrap_or(line);
        let line = line.strip_suffix(b"\r").unwrap_or(line);

        Self {
            buf: line.to_vec(),
            lines: std::iter::once(0..line.len()).collect(),
        }
    }

    /// The lines as lossily-decoded text, borrowing from the batch buffer
    /// unless a line contains invalid UTF-8.
    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = Cow<'_, str>> {
        self.lines
            .iter()
            .map(|range| String::from_utf8_lossy(&self.buf[range.clone()]))
    }
}

fn strip_cr(buf: &[u8], range: Range<usize>) -> Range<usize> {
    if buf[range.clone()].ends_with(b"\r") {
        range.start..range.end - 1
    } else {
        range
    }
}

#[derive(Debug)]
pub enum ReadResult {
    Eof,
    Batch(LineBatch),
}

/// Returns the complete lines already buffered in `reader` as a `Batch`,
/// awaiting more input if the buffered line is partial.
pub fn read_batch<R>(reader: &mut R) -> Result<ReadResult>
where
    R: BufRead,
{
    let buf = reader.fill_buf()?;

    if buf.is_empty() {
        return Ok(ReadResult::Eof);
    }

    // A line spanning the whole buffer (or an unterminated final line):
    // block until its newline or EOF arrives.
    let Some(last) = memrchr(b'\n', buf) else {
        let mut line = Vec::new();
        reader.read_until(b'\n', &mut line)?;

        return Ok(ReadResult::Batch(LineBatch::single_line(&line)));
    };

    let batch = LineBatch::from_chunk(&buf[..last]);
    reader.consume(last + 1);

    Ok(ReadResult::Batch(batch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read};
    use std::sync::mpsc;
    use std::time::Duration;

    fn texts(batch: &LineBatch) -> Vec<String> {
        batch.iter().map(Cow::into_owned).collect()
    }

    #[test]
    fn test_read_complete_lines_multiple_lines() {
        let input = b"line 1\nline 2\nline 3\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["line 1", "line 2", "line 3"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_do_not_omit_empty_lines() {
        let input = b"line 1\n\nline 3\n\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["line 1", "", "line 3", ""]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_multiple_lines_no_final_newline() {
        let input = b"line 1\nline 2\nline 3";
        let mut reader = BufReader::new(&input[..]);
        let mut lines = vec![];

        let lines_first_read = read_batch(&mut reader).unwrap();
        let lines_second_read = read_batch(&mut reader).unwrap();

        if let ReadResult::Batch(batch) = lines_first_read {
            lines.extend(texts(&batch));
        }

        if let ReadResult::Batch(batch) = lines_second_read {
            lines.extend(texts(&batch));
        }

        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
    }

    #[test]
    fn test_read_eof_without_new_line_should_still_return_the_whole_string() {
        let input = b"incomplete line without newline";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["incomplete line without newline"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_single_complete_line() {
        let input = b"single complete line\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["single complete line"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_empty_input() {
        let input = b"";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Eof) = read_batch(&mut reader) {
            // success
        } else {
            panic!("Expected Ok(ReadResult::Eof), got something else");
        }
    }

    #[test]
    fn test_read_complete_lines_partial_line_pending() {
        let rx = read_batch_in_background(b"incomplete line without newline");

        let result = rx.recv_timeout(Duration::from_millis(100));

        assert!(result.is_err(), "Expected read_batch to block without newline or EOF");
    }

    #[test]
    fn test_read_available_lines() {
        let rx = read_batch_in_background(b"extract lines\n that are ready\n to be extracted\n but not more than that");

        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(Ok(ReadResult::Batch(batch))) => {
                assert_eq!(
                    texts(&batch),
                    vec!["extract lines", " that are ready", " to be extracted"]
                );
            }
            other => panic!("Expected Ok(ReadResult::Batch), got {other:?}"),
        }
    }

    /// Runs `read_batch` over a source that blocks forever once its initial
    /// data is consumed, so tests can assert on blocking behavior with
    /// timeouts. A still-blocked thread dies with the test process.
    fn read_batch_in_background(input: &[u8]) -> mpsc::Receiver<Result<ReadResult>> {
        let source = BlocksAfterInitialRead::new(input);
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let mut reader = BufReader::new(source);
            let _ = tx.send(read_batch(&mut reader));
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

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["line1", "line2", "line3"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_crlf_single_line() {
        let input = b"line1\r\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["line1"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[test]
    fn test_non_utf8_single_line() {
        let input = b"caf\xe9\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            let lines = texts(&batch);
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

        if let Ok(ReadResult::Batch(batch)) = read_batch(&mut reader) {
            assert_eq!(texts(&batch), vec!["unix", "windows", "unix2"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }
}
