use miette::{IntoDiagnostic, Result};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Reads all **currently available** complete lines from reader.
/// If there’s no newline in the buffer, it will block until
/// at least one newline is available (or until EOF).
/// Leaves any incomplete trailing line in the buffer.
pub async fn read_complete_lines<R>(reader: &mut R) -> Result<Vec<String>>
where
    R: AsyncBufRead + Unpin,
{
    let mut lines = Vec::new();

    loop {
        match check_buffer(reader).await? {
            BufferState::Eof(buf) => {
                if !buf.is_empty() {
                    let leftover = String::from_utf8_lossy(buf).to_string();
                    let buf_len = buf.len();
                    reader.consume(buf_len);
                    lines.push(leftover);
                }
                return Ok(lines);
            }
            BufferState::NoNewline => {
                if block_until_newline(reader).await? == 0 {
                    return Ok(lines);
                }
            }
            BufferState::HasNewline(buf) => {
                let consumed = parse_lines(buf, &mut lines);
                reader.consume(consumed);
                return Ok(lines);
            }
        }
    }
}

enum BufferState<'a> {
    Eof(&'a [u8]),
    NoNewline,
    HasNewline(&'a [u8]),
}

async fn check_buffer<'a, R>(reader: &'a mut R) -> Result<BufferState<'a>>
where
    R: AsyncBufRead + Unpin,
{
    let buf = reader.fill_buf().await.into_diagnostic()?;

    if buf.is_empty() {
        Ok(BufferState::Eof(buf))
    } else if buf.contains(&b'\n') {
        Ok(BufferState::HasNewline(buf))
    } else {
        Ok(BufferState::NoNewline)
    }
}

async fn block_until_newline<R>(reader: &mut R) -> Result<usize>
where
    R: AsyncBufRead + Unpin,
{
    let mut discard = Vec::new();

    reader.read_until(b'\n', &mut discard).await.into_diagnostic()
}

fn parse_lines(buf: &[u8], lines: &mut Vec<String>) -> usize {
    let mut consumed = 0;

    while let Some(pos) = buf[consumed..].iter().position(|&b| b == b'\n') {
        let line_bytes = &buf[consumed..consumed + pos];
        lines.push(String::from_utf8_lossy(line_bytes).to_string());
        consumed += pos + 1;
    }

    consumed
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

        let lines = read_complete_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
    }

    #[tokio::test]
    async fn test_read_complete_lines_multiple_lines_no_final_newline() {
        let input = b"line 1\nline 2\nline 3";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_complete_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
    }

    #[tokio::test]
    async fn test_read_eof_without_new_line_should_still_return_the_whole_string() {
        let input = b"incomplete line without newline";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_complete_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["incomplete line without newline"]);
    }

    #[tokio::test]
    async fn test_read_complete_lines_single_complete_line() {
        let input = b"single complete line\n";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_complete_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["single complete line"]);
    }

    #[tokio::test]
    async fn test_read_complete_lines_empty_input() {
        let input = b"";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_complete_lines(&mut reader).await.unwrap();

        assert!(lines.is_empty());
    }

    #[tokio::test]
    async fn test_read_complete_lines_partial_line_pending() {
        let input = b"incomplete line without newline";
        let custom_reader = PendingAfterInitialRead::new(input);
        let mut reader = BufReader::new(custom_reader);

        let result = timeout(Duration::from_millis(100), read_complete_lines(&mut reader)).await;

        assert!(result.is_err(), "Expected timeout because no newline or EOF is present");
    }

    #[tokio::test]
    async fn test_read_available_lines() {
        let input = b"extract lines\n that are ready\n to be extracted\n but not more than that";
        let custom_reader = PendingAfterInitialRead::new(input);
        let mut reader = BufReader::new(custom_reader);

        let result = timeout(Duration::from_millis(100), read_complete_lines(&mut reader)).await;
        let lines = result.unwrap().unwrap();

        assert_eq!(lines, vec!["extract lines", " that are ready", " to be extracted"]);
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
                // After initial data is read, simulate a stream that's pending indefinitely
                Poll::Pending
            }
        }
    }
}
