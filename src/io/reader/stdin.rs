use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};
use tokio::sync::oneshot::Sender;

pub struct StdinReader {
    reader: BufReader<Stdin>,
    reached_eof_tx: Option<Sender<()>>,
    bucket_size: usize,
}

impl StdinReader {
    pub fn get_reader(reached_eof_tx: Option<Sender<()>>, bucket_size: usize) -> Box<dyn AsyncLineReader + Send> {
        Box::new(StdinReader {
            reader: BufReader::new(tokio::io::stdin()),
            reached_eof_tx,
            bucket_size,
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
        let mut bucket = Vec::new();

        while bucket.len() <= self.bucket_size {
            let buffer = match self.read_bytes_until_newline().await {
                Ok(buffer) if !buffer.is_empty() => buffer,
                _ => {
                    if !bucket.is_empty() {
                        self.send_eof_signal();
                    }
                    break;
                }
            };

            let buffer = Self::strip_newline_character(buffer);
            let line = String::from_utf8_lossy(&buffer).into_owned();

            bucket.push(line);
        }

        if bucket.is_empty() {
            Ok(None)
        } else {
            Ok(Some(bucket))
        }
    }
}
