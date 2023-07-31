use crate::reader::AsyncLineReader;
use async_trait::async_trait;
use linemux::MuxedLines;
use std::io;
use tokio::sync::oneshot::Sender;

pub struct Linemux {
    custom_message: Option<String>,
    number_of_lines: Option<usize>,
    current_line: usize,
    reached_eof_tx: Option<Sender<()>>,
    lines: MuxedLines,
}

impl Linemux {
    pub async fn get_reader_single(
        file_path: String,
        number_of_lines: usize,
        reached_eof_tx: Option<Sender<()>>,
    ) -> Box<dyn AsyncLineReader + Send> {
        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        lines
            .add_file_from_start(&file_path)
            .await
            .expect("Could not add file to linemux");

        Box::new(Self {
            custom_message: None,
            number_of_lines: Some(number_of_lines),
            current_line: 1,
            reached_eof_tx,
            lines,
        })
    }

    // For multiple files
    pub async fn get_reader_multiple(
        folder_name: String,
        file_paths: Vec<String>,
        mut reached_eof_tx: Option<Sender<()>>,
    ) -> Box<dyn AsyncLineReader + Send> {
        if let Some(reached_eof) = reached_eof_tx.take() {
            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }

        let mut lines = MuxedLines::new().expect("Could not instantiate linemux");

        for file_path in file_paths {
            lines
                .add_file(&file_path)
                .await
                .expect("Could not add file to linemux");
        }

        Box::new(Self {
            custom_message: Some("tailing folders".to_string()),
            number_of_lines: None,
            current_line: 1,
            reached_eof_tx,
            lines,
        })
    }

    fn send_eof_signal(&mut self) {
        if let Some(reached_eof) = self.reached_eof_tx.take() {
            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }
    }
}

#[async_trait]
impl AsyncLineReader for Linemux {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        if let Some(custom_message) = self.custom_message.take() {
            return Ok(Some(custom_message));
        }

        let line = match self.lines.next_line().await {
            Ok(Some(line)) => line,
            _ => return Ok(None),
        };

        if let Some(number_of_lines) = self.number_of_lines {
            if self.current_line == number_of_lines {
                self.send_eof_signal();
            }
        }

        self.current_line += 1;
        Ok(Some(line.line().to_owned()))
    }
}
