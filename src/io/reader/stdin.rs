use crate::eof_signal::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result};
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
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next_line_batch(&mut self) -> Result<Option<Vec<String>>> {
        let buffer = self
            .read_bytes_until_newline()
            .await
            .into_diagnostic()
            .wrap_err("Could not read from stdin")?;

        if buffer.is_empty() {
            self.initial_read_complete_sender.send()?;

            return Ok(None);
        }

        let buffer = Self::strip_newline_character(buffer);
        let line = String::from_utf8_lossy(&buffer).into_owned();

        Ok(Some(vec![line]))
    }
}
