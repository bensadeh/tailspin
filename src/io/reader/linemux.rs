use crate::io::controller::Reader;
use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use linemux::MuxedLines;
use std::io;
use std::path::PathBuf;
use tokio::sync::oneshot::Sender;

pub struct Linemux {
    number_of_lines: Option<usize>,
    current_line: usize,
    reached_eof_tx: Option<Sender<()>>,
    reached_eof: bool,
    lines: MuxedLines,
}

impl Linemux {
    pub async fn get_reader(file_path: PathBuf, number_of_lines: usize, reached_eof_tx: Option<Sender<()>>) -> Reader {
        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        lines
            .add_file_from_start(&file_path)
            .await
            .expect("Could not add file to linemux");

        Reader::Linemux(Self {
            number_of_lines: Some(number_of_lines),
            current_line: 0,
            reached_eof_tx,
            reached_eof: false,
            lines,
        })
    }

    fn send_eof_signal(&mut self) {
        if let Some(reached_eof) = self.reached_eof_tx.take() {
            self.reached_eof = true;

            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }
    }

    async fn read_lines_until_eof(&mut self) -> io::Result<Option<Vec<String>>> {
        let mut bucket = Vec::new();
        let total_lines = self.number_of_lines.expect("Number of lines not set");

        while bucket.len() < total_lines {
            let line = match self.lines.next_line().await {
                Ok(Some(line)) => line,
                _ => break,
            };

            bucket.push(line.line().to_owned());
            self.current_line += 1;

            if self.current_line >= total_lines {
                self.send_eof_signal();
            }
        }

        if bucket.is_empty() { Ok(None) } else { Ok(Some(bucket)) }
    }

    async fn read_line_by_line(&mut self) -> io::Result<Option<Vec<String>>> {
        let line = match self.lines.next_line().await {
            Ok(Some(line)) => line,
            _ => return Ok(None),
        };

        let next_line = line.line().to_owned();

        Ok(Some(vec![next_line]))
    }
}

#[async_trait]
impl AsyncLineReader for Linemux {
    async fn next_line_batch(&mut self) -> io::Result<Option<Vec<String>>> {
        match self.reached_eof {
            true => self.read_line_by_line().await,
            false => self.read_lines_until_eof().await,
        }
    }
}
