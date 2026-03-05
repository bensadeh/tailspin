use memchr::memchr;
use miette::{IntoDiagnostic, Result, miette};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pub const BUFF_READER_CAPACITY: usize = 64 * 1024;

pub enum ReadResult {
    Eof,
    Line(String),
    Batch(Vec<String>),
}

/// Peeks at the buffered reader to classify available data, then dispatches
/// to a handler. The peek/handle split exists because `fill_buf()` borrows
/// the reader — we must drop that borrow before calling `read_until` or
/// `consume` in the handlers.
pub async fn read_lines<R>(reader: &mut R) -> Result<ReadResult>
where
    R: AsyncBufRead + Unpin,
{
    match peek_buffer(reader).await? {
        PeekResult::Eof => Ok(ReadResult::Eof),
        PeekResult::SingleOrNoNewline => handle_single_or_no_newline(reader).await,
        PeekResult::MultipleNewlines => handle_multiple_newlines(reader).await,
    }
}

enum PeekResult {
    Eof,
    SingleOrNoNewline,
    MultipleNewlines,
}

async fn peek_buffer<R>(reader: &mut R) -> Result<PeekResult>
where
    R: AsyncBufRead + Unpin,
{
    let buf = reader.fill_buf().await.into_diagnostic()?;

    if buf.is_empty() {
        return Ok(PeekResult::Eof);
    }

    match memchr(b'\n', buf) {
        None => Ok(PeekResult::SingleOrNoNewline),
        Some(pos) => match memchr(b'\n', &buf[pos + 1..]) {
            None => Ok(PeekResult::SingleOrNoNewline),
            Some(_) => Ok(PeekResult::MultipleNewlines),
        },
    }
}

async fn handle_single_or_no_newline<R>(reader: &mut R) -> Result<ReadResult>
where
    R: AsyncBufRead + Unpin,
{
    let mut buf = Vec::new();
    reader.read_until(b'\n', &mut buf).await.into_diagnostic()?;

    if buf.last() == Some(&b'\n') {
        buf.pop();
    }
    if buf.last() == Some(&b'\r') {
        buf.pop();
    }

    let line = String::from_utf8_lossy(&buf).to_string();
    Ok(ReadResult::Line(line))
}

async fn handle_multiple_newlines<R>(reader: &mut R) -> Result<ReadResult>
where
    R: AsyncBufRead + Unpin,
{
    let buf = reader.fill_buf().await.into_diagnostic()?;

    if let Some(last_newline_pos) = buf.iter().rposition(|&b| b == b'\n') {
        let consumed_buf = &buf[..=last_newline_pos];
        let lines = parse_buffer(consumed_buf);
        reader.consume(last_newline_pos + 1);

        return Ok(ReadResult::Batch(lines));
    }

    Err(miette!("Expected multiple newlines, but none found"))
}

fn parse_buffer(buf: &[u8]) -> Vec<String> {
    let mut parts = buf
        .split(|&b| b == b'\n')
        .map(|slice| {
            let slice = slice.strip_suffix(b"\r").unwrap_or(slice);
            String::from_utf8_lossy(slice).to_string()
        })
        .collect::<Vec<String>>();

    if let Some(last) = parts.last() {
        if last.is_empty() {
            parts.pop();
        }
    }

    parts
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
