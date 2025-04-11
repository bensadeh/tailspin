use crate::eof_signal::EofSignalSender;
use crate::io::controller::Reader;
use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

pub struct StdinReader {
    reader: BufReader<Stdin>,
    eof_signal_sender: EofSignalSender,
}

impl StdinReader {
    pub fn get_reader(eof_signal_sender: EofSignalSender) -> Reader {
        Reader::Stdin(StdinReader {
            reader: BufReader::new(tokio::io::stdin()),
            eof_signal_sender,
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
        self.eof_signal_sender
            .send()
            .expect("Failed sending EOF signal to oneshot channel");
    }
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next_line_batch(&mut self) -> io::Result<Option<Vec<String>>> {
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
