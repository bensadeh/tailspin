use crate::eof_signal::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

pub struct StdinReader {
    reader: BufReader<Stdin>,
    initial_read_complete_sender: InitialReadCompleteSender,
}

impl StdinReader {
    pub fn get_reader(initial_read_complete_sender: InitialReadCompleteSender) -> Reader {
        Reader::Stdin(StdinReader {
            reader: BufReader::new(tokio::io::stdin()),
            initial_read_complete_sender,
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
        self.initial_read_complete_sender
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
