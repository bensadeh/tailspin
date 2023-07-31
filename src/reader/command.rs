use crate::reader::AsyncLineReader;
use async_trait::async_trait;
use std::process::{Command, Stdio};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;
use tokio::sync::oneshot::Sender;

pub struct CommandReader {
    reader: BufReader<tokio::process::ChildStdout>,
    reached_eof_tx: Option<Sender<()>>,
}

impl CommandReader {
    pub async fn get_reader(
        command: &str,
        reached_eof_tx: Option<Sender<()>>,
    ) -> io::Result<Box<dyn AsyncLineReader + Send>> {
        let trap_command = format!("trap '' INT; {}", command);

        let child = AsyncCommand::new("sh")
            .arg("-c")
            .arg(trap_command)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Could not capture standard output.")
        })?;

        let reader = BufReader::new(stdout);

        Ok(Box::new(CommandReader {
            reader,
            reached_eof_tx,
        }))
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
impl AsyncLineReader for CommandReader {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        let buffer = self.read_bytes_until_newline().await?;

        if buffer.is_empty() {
            self.send_eof_signal();
            return Ok(None);
        }

        let buffer = Self::strip_newline_character(buffer);
        let line = String::from_utf8_lossy(&buffer).into_owned();

        Ok(Some(line))
    }
}
