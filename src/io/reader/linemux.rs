use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use linemux::MuxedLines;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::path::PathBuf;

pub struct Linemux {
    number_of_lines: Option<usize>,
    current_line: usize,
    initial_read_complete_sender: InitialReadCompleteSender,
    reached_eof: bool,
    lines: MuxedLines,
}

impl Linemux {
    pub async fn get_reader(
        file_path: PathBuf,
        number_of_lines: usize,
        irc_sender: InitialReadCompleteSender,
    ) -> Reader {
        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        lines
            .add_file_from_start(&file_path)
            .await
            .expect("Could not add file to linemux");

        Reader::Linemux(Self {
            number_of_lines: Some(number_of_lines),
            current_line: 0,
            initial_read_complete_sender: irc_sender,
            reached_eof: false,
            lines,
        })
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

            if self.current_line >= total_lines {
                self.send_eof_signal()?;
            }
        }

        match bucket.is_empty() {
            true => Err(miette!("Error batching line reads from file")),
            false => Ok(ReadType::MultipleLines(bucket)),
        }
    }

    fn send_eof_signal(&mut self) -> Result<()> {
        self.reached_eof = true;

        self.initial_read_complete_sender.send()
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
