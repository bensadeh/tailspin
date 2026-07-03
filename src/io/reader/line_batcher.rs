use anyhow::Result;
use memchr::{memchr, memrchr};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pub const BUF_READER_CAPACITY: usize = 64 * 1024;

pub enum ReadResult {
    Eof,
    Line(String),
    Batch(Vec<String>),
}

/// Returns the complete lines already buffered in `reader`: a `Batch` when
/// several are available, otherwise a single `Line` (awaiting more input if
/// the buffered line is partial).
pub async fn read_lines<R>(reader: &mut R) -> Result<ReadResult>
where
    R: AsyncBufRead + Unpin,
{
    let buf = reader.fill_buf().await?;

    if buf.is_empty() {
        return Ok(ReadResult::Eof);
    }

    match memrchr(b'\n', buf) {
        Some(last) if memchr(b'\n', &buf[..last]).is_some() => {
            let lines = split_lines(&buf[..last]);
            reader.consume(last + 1);

            Ok(ReadResult::Batch(lines))
        }
        _ => read_line(reader).await,
    }
}

async fn read_line<R>(reader: &mut R) -> Result<ReadResult>
where
    R: AsyncBufRead + Unpin,
{
    let mut buf = Vec::new();
    reader.read_until(b'\n', &mut buf).await?;

    Ok(ReadResult::Line(decode_line(&buf)))
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
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;
    use tokio::io::BufReader;
    use tokio::io::{AsyncRead, ReadBuf};
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_read_complete_lines_multiple_lines() {
        let input = b"line 1\nline 2\nline 3\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader).await {
            assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[tokio::test]
    async fn test_do_not_omit_empty_lines() {
        let input = b"line 1\n\nline 3\n\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader).await {
            assert_eq!(lines, vec!["line 1", "", "line 3", ""]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[tokio::test]
    async fn test_read_complete_lines_multiple_lines_no_final_newline() {
        let input = b"line 1\nline 2\nline 3";
        let mut reader = BufReader::new(&input[..]);
        let mut lines = vec![];

        let lines_first_read = read_lines(&mut reader).await.unwrap();
        let lines_second_read = read_lines(&mut reader).await.unwrap();

        if let ReadResult::Batch(batch) = lines_first_read {
            lines = [lines, batch].concat();
        }

        if let ReadResult::Line(line) = lines_second_read {
            lines.push(line);
        }

        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
    }

    #[tokio::test]
    async fn test_read_eof_without_new_line_should_still_return_the_whole_string() {
        let input = b"incomplete line without newline";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Line(line)) = read_lines(&mut reader).await {
            assert_eq!(line, "incomplete line without newline");
        } else {
            panic!("Expected Ok(ReadResult::Line), got something else");
        }
    }

    #[tokio::test]
    async fn test_read_complete_lines_single_complete_line() {
        let input = b"single complete line\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Line(line)) = read_lines(&mut reader).await {
            assert_eq!(line, "single complete line");
        } else {
            panic!("Expected Ok(ReadResult::Line), got something else");
        }
    }

    #[tokio::test]
    async fn test_read_complete_lines_empty_input() {
        let input = b"";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Eof) = read_lines(&mut reader).await {
            // success
        } else {
            panic!("Expected Ok(ReadResult::Eof), got something else");
        }
    }

    #[tokio::test]
    async fn test_read_complete_lines_partial_line_pending() {
        let input = b"incomplete line without newline";
        let custom_reader = PendingAfterInitialRead::new(input);
        let mut reader = BufReader::new(custom_reader);

        let result = timeout(Duration::from_millis(100), read_lines(&mut reader)).await;

        assert!(result.is_err(), "Expected timeout because no newline or EOF is present");
    }

    #[tokio::test]
    async fn test_read_available_lines() {
        let input = b"extract lines\n that are ready\n to be extracted\n but not more than that";
        let custom_reader = PendingAfterInitialRead::new(input);
        let mut reader = BufReader::new(custom_reader);

        let result = timeout(Duration::from_millis(100), read_lines(&mut reader)).await;

        if let Ok(Ok(ReadResult::Batch(lines))) = result {
            assert_eq!(lines, vec!["extract lines", " that are ready", " to be extracted"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    struct PendingAfterInitialRead {
        data: Vec<u8>,
        position: usize,
    }

    impl PendingAfterInitialRead {
        fn new(initial_data: &[u8]) -> Self {
            Self {
                data: initial_data.to_vec(),
                position: 0,
            }
        }
    }

    impl AsyncRead for PendingAfterInitialRead {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            if self.position < self.data.len() {
                let remaining = &self.data[self.position..];
                let to_read = remaining.len().min(buf.remaining());
                buf.put_slice(&remaining[..to_read]);
                self.position += to_read;
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        }
    }

    #[tokio::test]
    async fn test_crlf_batch() {
        let input = b"line1\r\nline2\r\nline3\r\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader).await {
            assert_eq!(lines, vec!["line1", "line2", "line3"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }

    #[tokio::test]
    async fn test_crlf_single_line() {
        let input = b"line1\r\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Line(line)) = read_lines(&mut reader).await {
            assert_eq!(line, "line1");
        } else {
            panic!("Expected Ok(ReadResult::Line), got something else");
        }
    }

    #[tokio::test]
    async fn test_non_utf8_single_line() {
        let input = b"caf\xe9\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Line(line)) = read_lines(&mut reader).await {
            assert!(line.starts_with("caf"));
            assert!(line.contains('\u{FFFD}'));
        } else {
            panic!("Expected Ok(ReadResult::Line), got something else");
        }
    }

    #[tokio::test]
    async fn test_mixed_line_endings() {
        let input = b"unix\nwindows\r\nunix2\n";
        let mut reader = BufReader::new(&input[..]);

        if let Ok(ReadResult::Batch(lines)) = read_lines(&mut reader).await {
            assert_eq!(lines, vec!["unix", "windows", "unix2"]);
        } else {
            panic!("Expected Ok(ReadResult::Batch), got something else");
        }
    }
}
