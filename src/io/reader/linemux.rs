use crate::io::reader::StreamEvent::{Ended, Started};
use crate::io::reader::{AsyncLineReader, StreamEvent};
use async_trait::async_trait;
use linemux::MuxedLines;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::path::PathBuf;

pub struct Linemux {
    number_of_lines: usize,
    current_line_count: usize,
    has_read_all_initial_lines: bool,
    has_emitted_start_event: bool,
    lines: MuxedLines,
    terminate_after_first_read: bool,
}

impl Linemux {
    pub async fn new(file_path: PathBuf, number_of_lines: usize, terminate_after_first_read: bool) -> Result<Linemux> {
        let mut lines = MuxedLines::new()
            .into_diagnostic()
            .wrap_err("Could not instantiate linemux")?;

        lines
            .add_file_from_start(&file_path)
            .await
            .into_diagnostic()
            .wrap_err("Could not add file to linemux")?;

        Ok(Self {
            number_of_lines,
            current_line_count: 0,
            has_read_all_initial_lines: false,
            has_emitted_start_event: false,
            terminate_after_first_read,
            lines,
        })
    }

    async fn read_lines_until_eof(&mut self) -> Result<StreamEvent> {
        let mut lines = Vec::new();

        while lines.len() < self.number_of_lines {
            let next_line = self
                .lines
                .next_line()
                .await
                .into_diagnostic()
                .wrap_err("Could not read next line")?;

            let line = match next_line {
                Some(line) => line.line().to_string(),
                _ => break,
            };

            lines.push(line);
            self.current_line_count += 1;
        }

        Ok(StreamEvent::Lines(lines))
    }

    async fn read_line_by_line(&mut self) -> Result<StreamEvent> {
        let next_line = self
            .lines
            .next_line()
            .await
            .into_diagnostic()
            .wrap_err("Could not read next line")?
            .ok_or(miette!("next_line() should never return optional"))?;

        Ok(StreamEvent::Line(next_line.line().to_string()))
    }
}

#[async_trait]
impl AsyncLineReader for Linemux {
    async fn next(&mut self) -> Result<StreamEvent> {
        if self.has_read_all_initial_lines && !self.has_emitted_start_event {
            self.has_emitted_start_event = true;

            return Ok(Started);
        }

        if self.has_read_all_initial_lines && self.has_emitted_start_event && self.terminate_after_first_read {
            return Ok(Ended);
        }

        if !self.has_read_all_initial_lines {
            let stream_event = self.read_lines_until_eof().await?;
            self.has_read_all_initial_lines = true;

            return Ok(stream_event);
        }

        self.read_line_by_line().await
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

        let mut linemux = Linemux::new(file_path.clone(), 2, false).await?;

        let event = linemux.next().await?;
        match event {
            Lines(lines) => {
                assert_eq!(lines.len(), 2);
                assert_eq!(lines[0], "line1");
                assert_eq!(lines[1], "line2");
            }
            _ => panic!("Expected StreamEvent::Lines(...)"),
        }

        let event = linemux.next().await?;
        match event {
            Started => {}
            _ => panic!("Expected StreamEvent::Started"),
        }

        let event = linemux.next().await?;
        match event {
            Line(line) => assert_eq!(line, "line3"),
            _ => panic!("Expected StreamEvent::Line(...)"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_terminate_after_first_read() -> Result<()> {
        let test_result = timeout(Duration::from_millis(1000), async {
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("test.log");

            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "only_line").unwrap();

            let mut linemux = Linemux::new(file_path.clone(), 1, true).await?;

            let first_event = linemux.next().await?;
            match first_event {
                Lines(lines) => {
                    assert_eq!(lines.len(), 1);
                    assert_eq!(lines[0], "only_line");
                }
                _ => panic!("Expected StreamEvent::Lines(...)"),
            }

            let second_event = linemux.next().await?;
            match second_event {
                Started => {}
                _ => panic!("Expected StreamEvent::Started"),
            }

            let third_event = linemux.next().await?;
            match third_event {
                Ended => {}
                _ => panic!("Expected StreamEvent::Ended"),
            }

            Ok(())
        })
        .await;

        test_result.unwrap_or_else(|_| Err(miette!("Test timed out!")))
    }

    #[tokio::test]
    async fn test_append_new_lines_after_initial_read() -> Result<()> {
        let dir = tempdir().into_diagnostic()?;
        let file_path = dir.path().join("test_append.log");

        let mut file = File::create(&file_path).into_diagnostic()?;
        writeln!(file, "initial1").into_diagnostic()?;
        writeln!(file, "initial2").into_diagnostic()?;

        let mut linemux = Linemux::new(file_path.clone(), 2, false).await?;
        let event = linemux.next().await?;
        match event {
            Lines(lines) => {
                assert_eq!(lines.len(), 2);
                assert_eq!(lines[0], "initial1");
                assert_eq!(lines[1], "initial2");
            }
            _ => panic!("Expected StreamEvent::Lines(...)"),
        }

        let event = linemux.next().await?;
        assert!(matches!(event, Started));

        let mut file = OpenOptions::new().append(true).open(&file_path).into_diagnostic()?;
        writeln!(file, "appended1").into_diagnostic()?;
        writeln!(file, "appended2").into_diagnostic()?;

        sleep(Duration::from_millis(100)).await;

        let event = linemux.next().await?;
        match event {
            Line(line) => assert_eq!(line, "appended1"),
            _ => panic!("Expected StreamEvent::Line(...) with appended1"),
        }

        let event = linemux.next().await?;
        match event {
            Line(line) => assert_eq!(line, "appended2"),
            _ => panic!("Expected StreamEvent::Line(...) with appended2"),
        }

        Ok(())
    }
}
