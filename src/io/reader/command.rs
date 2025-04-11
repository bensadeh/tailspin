use crate::eof_signal::EofSignalSender;
use crate::io::controller::Reader;
use crate::io::reader::AsyncLineReader;
use async_trait::async_trait;
use std::process::Stdio;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;

pub struct CommandReader {
    reader: BufReader<tokio::process::ChildStdout>,
}

impl CommandReader {
    pub async fn get_reader(command: String, reached_eof_tx: EofSignalSender) -> Reader {
        reached_eof_tx
            .send()
            .expect("Failed sending EOF signal to oneshot channel");

        let trap_command = format!("trap '' INT; {}", command);

        let child = AsyncCommand::new("sh")
            .arg("-c")
            .arg(trap_command)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Could not spawn process");

        let stdout = child.stdout.expect("Could not spawn child process");

        let reader = BufReader::new(stdout);

        Reader::Command(CommandReader { reader })
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
    async fn next_line_batch(&mut self) -> io::Result<Option<Vec<String>>> {
        let buffer = self.read_bytes_until_newline().await?;

        if buffer.is_empty() {
            return Ok(None);
        }

        let buffer = Self::strip_newline_character(buffer);
        let line = String::from_utf8_lossy(&buffer).into_owned();

        Ok(Some(vec![line]))
    }
}
