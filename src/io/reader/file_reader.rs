use crate::io::reader::StreamEvent;
use crate::io::reader::StreamEvent::{Ended, InitialReadComplete};
use crate::io::reader::line_batcher::{BUF_READER_CAPACITY, ReadResult, decode_line, read_lines};
use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

enum Stage {
    InitialRead,
    Following,
    Terminated,
}

pub struct FileReader {
    reader: BufReader<tokio::fs::File>,
    buf: Vec<u8>,
    stage: Stage,
    terminate_after_first_read: bool,
}

impl FileReader {
    pub async fn new<P: AsRef<Path>>(file_path: P, terminate_after_first_read: bool) -> Result<FileReader> {
        let file_path = std::fs::canonicalize(file_path.as_ref()).context("Could not canonicalize file path")?;

        let file = tokio::fs::File::open(&file_path).await.context("Could not open file")?;

        let reader = BufReader::with_capacity(BUF_READER_CAPACITY, file);

        Ok(Self {
            reader,
            buf: Vec::new(),
            stage: Stage::InitialRead,
            terminate_after_first_read,
        })
    }

    async fn next_line(&mut self) -> Result<String> {
        loop {
            let bytes_read = self
                .reader
                .read_until(b'\n', &mut self.buf)
                .await
                .context("Could not read next line")?;

            if bytes_read == 0 {
                // Detect file truncation: if the file shrank past our position, restart from the beginning
                let file_size = self
                    .reader
                    .get_ref()
                    .metadata()
                    .await
                    .context("Could not stat file")?
                    .len();
                let position = self
                    .reader
                    .stream_position()
                    .await
                    .context("Could not get stream position")?;

                if file_size < position {
                    self.reader
                        .seek(std::io::SeekFrom::Start(0))
                        .await
                        .context("Could not seek to start after truncation")?;
                    self.buf.clear();
                }

                tokio::time::sleep(POLL_INTERVAL).await;
                continue;
            }

            if self.buf.ends_with(b"\n") {
                let line = decode_line(&self.buf);
                self.buf.clear();
                return Ok(line);
            }

            // Partial line at EOF — wait for more data
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    pub async fn next(&mut self) -> Result<StreamEvent> {
        match self.stage {
            Stage::InitialRead => match read_lines(&mut self.reader).await? {
                ReadResult::Line(line) => Ok(StreamEvent::Line(line)),
                ReadResult::Batch(lines) => Ok(StreamEvent::Lines(lines)),
                ReadResult::Eof => {
                    self.stage = if self.terminate_after_first_read {
                        Stage::Terminated
                    } else {
                        Stage::Following
                    };
                    Ok(InitialReadComplete)
                }
            },
            Stage::Following => Ok(StreamEvent::Line(self.next_line().await?)),
            Stage::Terminated => Ok(Ended),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::reader::StreamEvent::*;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use tempfile::tempdir;
    use tokio::time::{Duration, sleep, timeout};

    #[tokio::test]
    async fn test_read_exactly_n_lines() -> Result<()> {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.log");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "line1").unwrap();
        writeln!(file, "line2").unwrap();
        writeln!(file, "line3").unwrap();

        let mut reader = FileReader::new(file_path, false).await?;

        let event = reader.next().await?;
        match event {
            Lines(lines) => {
                assert_eq!(lines.len(), 3);
                assert_eq!(lines[0], "line1");
                assert_eq!(lines[1], "line2");
                assert_eq!(lines[2], "line3");
            }
            _ => panic!("Expected StreamEvent::Lines(...)"),
        }

        let event = reader.next().await?;
        match event {
            InitialReadComplete => {}
            _ => panic!("Expected StreamEvent::InitialReadComplete"),
        }

        let result = timeout(Duration::from_millis(200), reader.next()).await;

        assert!(
            result.is_err(),
            "Entire file has been read, next() should not return anything: {result:?}"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_terminate_after_first_read() -> Result<()> {
        let test_result = timeout(Duration::from_secs(1), async {
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("test.log");

            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "only_line").unwrap();

            let mut reader = FileReader::new(file_path, true).await?;

            let first_event = reader.next().await?;
            match first_event {
                Line(line) => assert_eq!(line, "only_line"),
                _ => panic!("Expected StreamEvent::Line(...)"),
            }

            let second_event = reader.next().await?;
            match second_event {
                InitialReadComplete => {}
                _ => panic!("Expected StreamEvent::InitialReadComplete"),
            }

            let third_event = reader.next().await?;
            match third_event {
                Ended => {}
                _ => panic!("Expected StreamEvent::Ended"),
            }

            Ok(())
        })
        .await;

        test_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Test timed out!")))
    }

    #[tokio::test]
    async fn test_append_new_lines_after_initial_read() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_append.log");

        let mut file = File::create(&file_path)?;
        writeln!(file, "initial1")?;
        writeln!(file, "initial2")?;

        let mut reader = FileReader::new(file_path.as_path(), false).await?;
        let event = reader.next().await?;
        match event {
            Lines(lines) => {
                assert_eq!(lines.len(), 2);
                assert_eq!(lines[0], "initial1");
                assert_eq!(lines[1], "initial2");
            }
            _ => panic!("Expected StreamEvent::Lines(...)"),
        }

        let event = reader.next().await?;
        assert!(matches!(event, InitialReadComplete));

        let mut file = OpenOptions::new().append(true).open(&file_path)?;
        writeln!(file, "appended1")?;
        writeln!(file, "appended2")?;

        sleep(Duration::from_millis(200)).await;

        let event = timeout(Duration::from_secs(1), reader.next())
            .await
            .context("Timed out waiting for appended1")?;
        match event? {
            Line(line) => assert_eq!(line, "appended1"),
            _ => panic!("Expected StreamEvent::Line(...) with appended1"),
        }

        let event = timeout(Duration::from_secs(1), reader.next())
            .await
            .context("Timed out waiting for appended2")?;
        match event? {
            Line(line) => assert_eq!(line, "appended2"),
            _ => panic!("Expected StreamEvent::Line(...) with appended2"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_file() -> Result<()> {
        let test_result = timeout(Duration::from_secs(1), async {
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("empty.log");
            File::create(&file_path).unwrap();

            let mut reader = FileReader::new(file_path, true).await?;

            let event = reader.next().await?;
            assert!(matches!(event, InitialReadComplete));

            let event = reader.next().await?;
            assert!(matches!(event, Ended));

            Ok(())
        })
        .await;

        test_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Test timed out!")))
    }

    #[tokio::test]
    async fn test_no_trailing_newline() -> Result<()> {
        let test_result = timeout(Duration::from_secs(1), async {
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("no_trailing.log");

            let mut file = File::create(&file_path).unwrap();
            write!(file, "line1\nline2").unwrap();

            let mut reader = FileReader::new(file_path, true).await?;

            let mut all_lines = Vec::new();
            loop {
                let event = reader.next().await?;
                match event {
                    Line(line) => all_lines.push(line),
                    Lines(lines) => all_lines.extend(lines),
                    InitialReadComplete | Ended => break,
                }
            }
            assert_eq!(all_lines, vec!["line1", "line2"]);

            Ok(())
        })
        .await;

        test_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Test timed out!")))
    }

    #[tokio::test]
    async fn test_crlf_line_endings() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("crlf.log");

        {
            let mut file = File::create(&file_path)?;
            file.write_all(b"line1\r\nline2\r\n")?;
        }

        let mut reader = FileReader::new(file_path.as_path(), false).await?;

        let event = reader.next().await?;
        match event {
            Lines(lines) => {
                assert_eq!(lines.len(), 2);
                assert_eq!(lines[0], "line1");
                assert_eq!(lines[1], "line2");
            }
            _ => panic!("Expected StreamEvent::Lines(...)"),
        }

        let event = reader.next().await?;
        assert!(matches!(event, InitialReadComplete));

        // Append a CRLF line in follow mode
        {
            let mut file = OpenOptions::new().append(true).open(&file_path)?;
            file.write_all(b"appended\r\n")?;
        }

        sleep(Duration::from_millis(200)).await;

        let event = timeout(Duration::from_secs(1), reader.next())
            .await
            .context("Timed out waiting for appended CRLF line")?;
        match event? {
            Line(line) => assert_eq!(line, "appended"),
            _ => panic!("Expected StreamEvent::Line(\"appended\")"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_non_utf8_content() -> Result<()> {
        let test_result = timeout(Duration::from_secs(1), async {
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("non_utf8.log");

            {
                let mut file = File::create(&file_path).unwrap();
                // Write invalid UTF-8: 0xFF 0xFE are not valid UTF-8 byte sequences
                file.write_all(b"hello \xff\xfe world\n").unwrap();
            }

            let mut reader = FileReader::new(file_path, true).await?;

            let event = reader.next().await?;
            match event {
                Line(line) => {
                    assert!(line.contains("hello"));
                    assert!(line.contains("world"));
                    assert!(line.contains('\u{FFFD}'));
                }
                _ => panic!("Expected StreamEvent::Line(...)"),
            }

            Ok(())
        })
        .await;

        test_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Test timed out!")))
    }

    #[tokio::test]
    async fn test_non_utf8_in_follow_mode() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("non_utf8_follow.log");

        {
            let mut file = File::create(&file_path)?;
            writeln!(file, "initial")?;
        }

        let mut reader = FileReader::new(file_path.as_path(), false).await?;

        let event = reader.next().await?;
        match event {
            Line(line) => assert_eq!(line, "initial"),
            _ => panic!("Expected StreamEvent::Line(\"initial\")"),
        }

        let event = reader.next().await?;
        assert!(matches!(event, InitialReadComplete));

        // Append non-UTF-8 in follow mode
        {
            let mut file = OpenOptions::new().append(true).open(&file_path)?;
            file.write_all(b"caf\xe9\n")?;
        }

        sleep(Duration::from_millis(200)).await;

        let event = timeout(Duration::from_secs(1), reader.next())
            .await
            .context("Timed out waiting for non-UTF-8 line")?;
        match event? {
            Line(line) => {
                assert!(line.starts_with("caf"));
                assert!(line.contains('\u{FFFD}'));
            }
            _ => panic!("Expected StreamEvent::Line(...)"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_truncation_detection() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("truncate.log");

        let mut file = File::create(&file_path)?;
        writeln!(file, "original1")?;
        writeln!(file, "original2")?;

        let mut reader = FileReader::new(file_path.as_path(), false).await?;

        let event = reader.next().await?;
        match event {
            Lines(lines) => {
                assert_eq!(lines.len(), 2);
                assert_eq!(lines[0], "original1");
                assert_eq!(lines[1], "original2");
            }
            _ => panic!("Expected StreamEvent::Lines(...)"),
        }

        let event = reader.next().await?;
        assert!(matches!(event, InitialReadComplete));

        // Truncate the file and write new, shorter content
        let mut file = File::create(&file_path)?;
        writeln!(file, "new")?;

        sleep(Duration::from_millis(200)).await;

        let event = timeout(Duration::from_secs(1), reader.next())
            .await
            .context("Timed out waiting for line after truncation")?;
        match event? {
            Line(line) => assert_eq!(line, "new"),
            _ => panic!("Expected StreamEvent::Line(\"new\")"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_large_file_streams_in_batches() -> Result<()> {
        let test_result = timeout(Duration::from_secs(5), async {
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("large.log");

            {
                let mut file = File::create(&file_path).unwrap();
                for i in 0..2000 {
                    writeln!(
                        file,
                        "line {i:05} - padding to make this line reasonably long for testing"
                    )
                    .unwrap();
                }
            }

            let mut reader = FileReader::new(file_path, true).await?;

            let mut event_count = 0;
            let mut total_lines = 0;

            loop {
                let event = reader.next().await?;
                match event {
                    Line(_) => {
                        event_count += 1;
                        total_lines += 1;
                    }
                    Lines(lines) => {
                        event_count += 1;
                        total_lines += lines.len();
                    }
                    InitialReadComplete => break,
                    Ended => panic!("Unexpected Ended before InitialReadComplete"),
                }
            }

            assert_eq!(total_lines, 2000);
            assert!(
                event_count > 1,
                "Large file should produce multiple events, got {event_count}"
            );

            Ok(())
        })
        .await;

        test_result.unwrap_or_else(|_| Err(anyhow::anyhow!("Test timed out!")))
    }
}
