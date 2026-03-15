use crate::io::reader::buffer_line_counter::{BUFF_READER_CAPACITY, ReadResult, read_lines};
use crate::io::reader::{AsyncLineReader, StreamEvent};
use anyhow::{Context, Result, anyhow};
use std::process::Stdio;
use tokio::io::BufReader;
use tokio::process::{Child, ChildStdout, Command};

pub struct CommandReader {
    reader: BufReader<ChildStdout>,
    child: Child,
    ready: bool,
}

impl CommandReader {
    pub async fn new(command: String) -> Result<CommandReader> {
        spawn_command(command).await
    }
}

#[cfg(not(windows))]
async fn spawn_command(command: String) -> Result<CommandReader> {
    let trap_command = format!("trap '' INT; {}", command);

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(trap_command)
        .stdout(Stdio::piped())
        .spawn()
        .context("Could not spawn process")?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("Could not capture stdout of spawned process"))?;

    let reader = BufReader::with_capacity(BUFF_READER_CAPACITY, stdout);

    Ok(CommandReader {
        reader,
        child,
        ready: false,
    })
}

#[cfg(windows)]
async fn spawn_command(_command: String) -> Result<CommandReader> {
    Err(anyhow!("The --exec flag is not supported on Windows"))
}

impl Drop for CommandReader {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

impl AsyncLineReader for CommandReader {
    async fn next(&mut self) -> Result<StreamEvent> {
        if !self.ready {
            self.ready = true;

            return Ok(StreamEvent::Started);
        }

        let event = match read_lines(&mut self.reader).await? {
            ReadResult::Eof => {
                let _ = self.child.wait().await;
                StreamEvent::Ended
            }
            ReadResult::Line(line) => StreamEvent::Line(line),
            ReadResult::Batch(lines) => StreamEvent::Lines(lines),
        };

        Ok(event)
    }
}
