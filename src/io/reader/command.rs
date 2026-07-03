use crate::io::reader::StreamEvent;
use crate::io::reader::line_batcher::{BUF_READER_CAPACITY, ReadResult, read_lines};
use anyhow::{Context, Result, anyhow, ensure};
use std::process::Stdio;
use tokio::io::BufReader;
use tokio::process::{Child, ChildStdout, Command};

pub struct CommandReader {
    reader: BufReader<ChildStdout>,
    child: Child,
    initial_read_complete_sent: bool,
}

impl CommandReader {
    pub async fn new(command: String) -> Result<CommandReader> {
        spawn_command(command).await
    }
}

#[cfg(not(windows))]
#[allow(clippy::unused_async)]
async fn spawn_command(command: String) -> Result<CommandReader> {
    let trap_command = format!("trap '' INT; {command}");

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

    let reader = BufReader::with_capacity(BUF_READER_CAPACITY, stdout);

    Ok(CommandReader {
        reader,
        child,
        initial_read_complete_sent: false,
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

impl CommandReader {
    pub async fn next(&mut self) -> Result<StreamEvent> {
        if !self.initial_read_complete_sent {
            self.initial_read_complete_sent = true;

            return Ok(StreamEvent::InitialReadComplete);
        }

        let event = match read_lines(&mut self.reader).await? {
            ReadResult::Eof => {
                let status = self.child.wait().await?;
                ensure!(status.success(), "--exec command failed ({status})");
                StreamEvent::Ended
            }
            ReadResult::Batch(lines) => StreamEvent::Lines(lines),
        };

        Ok(event)
    }
}
