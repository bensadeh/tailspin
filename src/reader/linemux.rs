use crate::reader::AsyncLineReader;
use async_trait::async_trait;
use linemux::MuxedLines;
use std::io;
use tokio::sync::oneshot::Sender;

pub struct Linemux {
    number_of_lines: usize,
    current_line: usize,
    reached_eof_tx: Option<Sender<()>>,
    lines: MuxedLines,
}

impl Linemux {
    pub async fn get_reader(
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
            number_of_lines,
            current_line: 1,
            reached_eof_tx,
            lines,
        })
    }
}

#[async_trait]
impl AsyncLineReader for Linemux {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        if let Ok(Some(line)) = self.lines.next_line().await {
            if self.current_line == self.number_of_lines {
                if let Some(reached_eof) = self.reached_eof_tx.take() {
                    reached_eof
                        .send(())
                        .expect("Failed sending EOF signal to oneshot channel");
                }
            }
            self.current_line += 1;
            return Ok(Some(line.line().to_owned()));
        }

        Ok(None)
    }
}
