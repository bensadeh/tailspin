use crate::io::reader::StreamEvent;
use crate::io::reader::line_batcher::{ReadResult, read_lines};
use anyhow::{Result, anyhow, ensure};
use shared_child::SharedChild;
use std::io::BufReader;
use std::process::ChildStdout;
use std::sync::Arc;

pub struct CommandReader {
    reader: BufReader<ChildStdout>,
    child: Arc<SharedChild>,
    initial_read_complete_sent: bool,
}

impl CommandReader {
    pub fn new(command: String) -> Result<CommandReader> {
        spawn_command(command)
    }

    pub fn child(&self) -> Arc<SharedChild> {
        self.child.clone()
    }
}

#[cfg(not(windows))]
fn spawn_command(command: String) -> Result<CommandReader> {
    use crate::io::reader::line_batcher::BUF_READER_CAPACITY;
    use anyhow::Context;
    use std::process::{Command, Stdio};

    let trap_command = format!("trap '' INT; {command}");

    let mut sh = Command::new("sh");
    sh.arg("-c").arg(trap_command).stdout(Stdio::piped());

    let child = SharedChild::spawn(&mut sh).context("Could not spawn process")?;

    let stdout = child
        .take_stdout()
        .ok_or_else(|| anyhow!("Could not capture stdout of spawned process"))?;

    let reader = BufReader::with_capacity(BUF_READER_CAPACITY, stdout);

    Ok(CommandReader {
        reader,
        child: Arc::new(child),
        initial_read_complete_sent: false,
    })
}

#[cfg(windows)]
fn spawn_command(_command: String) -> Result<CommandReader> {
    Err(anyhow!("The --exec flag is not supported on Windows"))
}

impl Drop for CommandReader {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

impl CommandReader {
    pub fn next(&mut self) -> Result<StreamEvent> {
        if !self.initial_read_complete_sent {
            self.initial_read_complete_sent = true;

            return Ok(StreamEvent::InitialReadComplete);
        }

        let event = match read_lines(&mut self.reader)? {
            ReadResult::Eof => {
                let status = self.child.wait()?;
                ensure!(status.success(), "--exec command failed ({status})");
                StreamEvent::Ended
            }
            ReadResult::Batch(lines) => StreamEvent::Lines(lines),
        };

        Ok(event)
    }
}
