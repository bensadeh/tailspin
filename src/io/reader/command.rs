use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::process::Stdio;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;

pub struct CommandReader {
    reader: BufReader<tokio::process::ChildStdout>,
}

impl CommandReader {
    pub async fn get_reader(command: String, mut reached_eof_tx: InitialReadCompleteSender) -> Result<Reader> {
        reached_eof_tx.send()?;

        let trap_command = format!("trap '' INT; {}", command);

        let child = AsyncCommand::new("sh")
            .arg("-c")
            .arg(trap_command)
            .stdout(Stdio::piped())
            .spawn()
            .into_diagnostic()
            .wrap_err("Could not spawn process")?;

        let stdout = child
            .stdout
            .ok_or_else(|| miette!("Could not capture stdout of spawned process"))?;

        let reader = BufReader::new(stdout);

        Ok(Reader::Command(CommandReader { reader }))
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
impl AsyncLineReader for CommandReader {
    async fn next(&mut self) -> Result<ReadType> {
        let buffer = self
            .read_bytes_until_newline()
            .await
            .into_diagnostic()
            .wrap_err("Could not read from stream")?;

        if buffer.is_empty() {
            return Ok(ReadType::StreamEnded);
        }

        let buffer = Self::strip_newline_character(buffer);
        let line = String::from_utf8_lossy(&buffer).into_owned();

        Ok(ReadType::SingleLine(line))
    }
}
