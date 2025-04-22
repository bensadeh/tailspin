use crate::io::reader::StreamEvent::{Ended, Started};
use crate::io::reader::{AsyncLineReader, StreamEvent};
use async_trait::async_trait;
use linemux::MuxedLines;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::path::PathBuf;

pub struct Linemux {
    number_of_lines: Option<usize>,
    current_line: usize,
    reached_eof: bool,
    stream_started: bool,
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
            number_of_lines: Some(number_of_lines),
            current_line: 0,
            reached_eof: false,
            stream_started: false,
            terminate_after_first_read,
            lines,
        })
    }

    async fn read_lines_until_eof(&mut self) -> Result<StreamEvent> {
        let mut lines = Vec::new();
        let total_lines = self.number_of_lines.expect("Number of lines not set");

        while lines.len() < total_lines {
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
            self.current_line += 1;
        }

        // self.reached_eof = true;

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
        if self.reached_eof && !self.stream_started {
            self.stream_started = true;

            return Ok(Started);
        }

        if self.reached_eof && self.stream_started && self.terminate_after_first_read {
            return Ok(Ended);
        }

        if !self.reached_eof {
            let stream_event = self.read_lines_until_eof().await?;
            self.reached_eof = true;

            return Ok(stream_event);
        }

        self.read_line_by_line().await
    }
}
