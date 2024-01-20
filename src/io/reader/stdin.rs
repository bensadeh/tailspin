use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use std::time::Duration;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};
use tokio::sync::oneshot::Sender;
use tokio::time::timeout;

pub struct StdinReader {
    reader: BufReader<Stdin>,
    reached_eof_tx: Option<Sender<()>>,
    is_first_call: bool,
}

impl StdinReader {
    pub fn get_reader(reached_eof_tx: Option<Sender<()>>) -> Box<dyn AsyncLineReader + Send> {
        Box::new(StdinReader {
            reader: BufReader::new(tokio::io::stdin()),
            reached_eof_tx,
            is_first_call: true,
        })
    }

    async fn read_bytes_until_newline(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        self.reader.read_until(b'\n', &mut buffer).await?;

        Ok(buffer)
    }

    fn strip_newline_character(buffer: Vec<u8>) -> Vec<u8> {
        let mut buf = buffer;

        if let Some(last_byte) = buf.last() {
            if *last_byte == b'\n' {
                buf.pop();
            }
        }

        buf
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
impl AsyncLineReader for StdinReader {
    async fn next_line(&mut self) -> io::Result<Option<Vec<String>>> {
        if self.is_first_call {
            self.is_first_call = false;
            let mut lines = Vec::new();
            let timeout_duration = Duration::from_millis(100);

            loop {
                match timeout(timeout_duration, self.read_bytes_until_newline()).await {
                    Ok(Ok(buffer)) if !buffer.is_empty() => {
                        let buffer = Self::strip_newline_character(buffer);
                        let line = String::from_utf8_lossy(&buffer).into_owned();
                        lines.push(line);
                    }
                    _ => break,
                }
            }

            if lines.is_empty() {
                self.send_eof_signal();
                return Ok(None);
            }

            return Ok(Some(lines));
        }

        let buffer = self.read_bytes_until_newline().await?;

        if buffer.is_empty() {
            self.send_eof_signal();
            return Ok(None);
        }

        let buffer = Self::strip_newline_character(buffer);
        let line = String::from_utf8_lossy(&buffer).into_owned();

        Ok(Some(vec![line]))
    }
}
