use miette::{IntoDiagnostic, Result};
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

pub async fn read_available_lines<R>(reader: &mut R) -> Result<Vec<String>>
where
    R: AsyncBufRead + Unpin,
{
    match peek_buffer(reader).await? {
        PeekResult::Eof(buf) => {
            let lines = parse_buffer(buf);
            let buf_len = buf.len();

            reader.consume(buf_len);

            Ok(lines)
        }
        PeekResult::SingleOrNoNewline => {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).await.into_diagnostic()?;
            if bytes_read == 0 {
                Ok(vec![])
            } else {
                Ok(vec![line.trim_end_matches('\n').to_string()])
            }
        }
        PeekResult::MultipleNewlines(buf) => {
            let last_newline_pos = buf
                .iter()
                .enumerate()
                .filter(|&(_, &b)| b == b'\n')
                .map(|(pos, _)| pos)
                .next_back()
                .unwrap();

            let consumed_buf = &buf[..=last_newline_pos];
            let lines = parse_buffer(consumed_buf);
            reader.consume(last_newline_pos + 1);

            Ok(lines)
        }
    }
}

enum PeekResult<'a> {
    Eof(&'a [u8]),
    SingleOrNoNewline,
    MultipleNewlines(&'a [u8]),
}

async fn peek_buffer<R>(reader: &mut R) -> Result<PeekResult>
where
    R: AsyncBufRead + Unpin,
{
    let buf = reader.fill_buf().await.into_diagnostic()?;

    if buf.is_empty() {
        return Ok(PeekResult::Eof(buf));
    }

    let newline_count = buf.iter().filter(|&&b| b == b'\n').count();

    match newline_count {
        0 | 1 => Ok(PeekResult::SingleOrNoNewline),
        _ => Ok(PeekResult::MultipleNewlines(buf)),
    }
}

fn parse_buffer(buf: &[u8]) -> Vec<String> {
    buf.split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| String::from_utf8_lossy(line).to_string())
        .collect()
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

        let lines = read_available_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["line 1", "line 2", "line 3"]);
    }

    #[tokio::test]
    async fn test_read_complete_lines_multiple_lines_no_final_newline() {
        let input = b"line 1\nline 2\nline 3";
        let mut reader = BufReader::new(&input[..]);

        let lines_first_read = read_available_lines(&mut reader).await.unwrap();
        let lines_second_read = read_available_lines(&mut reader).await.unwrap();

        assert_eq!(
            [lines_first_read, lines_second_read].concat(),
            vec!["line 1", "line 2", "line 3"]
        );
    }

    #[tokio::test]
    async fn test_read_eof_without_new_line_should_still_return_the_whole_string() {
        let input = b"incomplete line without newline";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_available_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["incomplete line without newline"]);
    }

    #[tokio::test]
    async fn test_read_complete_lines_single_complete_line() {
        let input = b"single complete line\n";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_available_lines(&mut reader).await.unwrap();

        assert_eq!(lines, vec!["single complete line"]);
    }

    #[tokio::test]
    async fn test_read_complete_lines_empty_input() {
        let input = b"";
        let mut reader = BufReader::new(&input[..]);

        let lines = read_available_lines(&mut reader).await.unwrap();

        assert!(lines.is_empty());
    }

    #[tokio::test]
    async fn test_read_complete_lines_partial_line_pending() {
        let input = b"incomplete line without newline";
        let custom_reader = PendingAfterInitialRead::new(input);
        let mut reader = BufReader::new(custom_reader);

        let result = timeout(Duration::from_millis(100), read_available_lines(&mut reader)).await;

        assert!(result.is_err(), "Expected timeout because no newline or EOF is present");
    }

    #[tokio::test]
    async fn test_read_available_lines() {
        let input = b"extract lines\n that are ready\n to be extracted\n but not more than that";
        let custom_reader = PendingAfterInitialRead::new(input);
        let mut reader = BufReader::new(custom_reader);

        let result = timeout(Duration::from_millis(100), read_available_lines(&mut reader)).await;
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
