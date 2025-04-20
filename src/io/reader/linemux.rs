use crate::io::controller::Reader;
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use linemux::MuxedLines;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::path::PathBuf;

pub struct Linemux {
    number_of_lines: Option<usize>,
    current_line: usize,
    reached_eof: bool,
    lines: MuxedLines,
    keep_alive: bool,
}

impl Linemux {
    pub async fn get_reader(file_path: PathBuf, number_of_lines: usize, keep_alive: bool) -> Result<Reader> {
        let mut lines = MuxedLines::new()
            .into_diagnostic()
            .wrap_err("Could not instantiate linemux")?;

        lines
            .add_file_from_start(&file_path)
            .await
            .into_diagnostic()
            .wrap_err("Could not add file to linemux")?;

        Ok(Reader::Linemux(Self {
            number_of_lines: Some(number_of_lines),
            current_line: 0,
            reached_eof: false,
            lines,
            keep_alive,
        }))
    }

    async fn read_lines_until_eof(&mut self) -> Result<ReadType> {
        let mut bucket = Vec::new();
        let total_lines = self.number_of_lines.expect("Number of lines not set");

        while bucket.len() < total_lines {
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

            bucket.push(line);
            self.current_line += 1;
        }

        self.reached_eof = true;

        if self.keep_alive {
            return Ok(ReadType::MultipleLines(bucket));
        }

        Ok(ReadType::InitialRead(bucket))
    }

    async fn read_line_by_line(&mut self) -> Result<ReadType> {
        let next_line = self
            .lines
            .next_line()
            .await
            .into_diagnostic()
            .wrap_err("Could not read next line")?
            .ok_or(miette!("next_line() should never return optional"))?;

        Ok(ReadType::SingleLine(next_line.line().to_string()))
    }
}

#[async_trait]
impl AsyncLineReader for Linemux {
    async fn next(&mut self) -> Result<ReadType> {
        match self.reached_eof {
            true => self.read_line_by_line().await,
            false => self.read_lines_until_eof().await,
        }
    }
}
