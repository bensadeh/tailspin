use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

use super::EOFSignaler;

pub struct StdinReader {
    reader: BufReader<Stdin>,
    eof_signaler: EOFSignaler,
}

impl StdinReader {
    pub fn get_reader(eof_signaler: EOFSignaler) -> Box<dyn AsyncLineReader + Send> {
        Box::new(StdinReader {
            reader: BufReader::new(tokio::io::stdin()),
            eof_signaler,
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
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        let buffer = self.read_bytes_until_newline().await?;

        if buffer.is_empty() {
            super::send_eof_signal(self.eof_signaler.take());
            return Ok(None);
        }

        let buffer = Self::strip_newline_character(buffer);
        let line = String::from_utf8_lossy(&buffer).into_owned();

        Ok(Some(line))
    }
}
