use crate::io::reader::common::{BUFF_READER_CAPACITY, ReadResult, read_lines};
use crate::io::reader::{AsyncLineReader, StreamEvent};
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::process::Stdio;
use tokio::io::BufReader;
use tokio::process::{ChildStdout, Command};

pub struct CommandReader {
    reader: BufReader<ChildStdout>,
    ready: bool,
}

impl CommandReader {
    pub async fn new(command: String) -> Result<CommandReader> {
        let trap_command = format!("trap '' INT; {}", command);

        let child = Command::new("sh")
            .arg("-c")
            .arg(trap_command)
            .stdout(Stdio::piped())
            .spawn()
            .into_diagnostic()
            .wrap_err("Could not spawn process")?;

        let stdout = child
            .stdout
            .ok_or_else(|| miette!("Could not capture stdout of spawned process"))?;

        let reader = BufReader::with_capacity(BUFF_READER_CAPACITY, stdout);

        Ok(CommandReader { reader, ready: false })
    }
}

#[async_trait]
impl AsyncLineReader for CommandReader {
    async fn next(&mut self) -> Result<StreamEvent> {
        if !self.ready {
            self.ready = !self.ready;

            return Ok(StreamEvent::Started);
        }

        read_lines(&mut self.reader).await.map(|res| match res {
            ReadResult::Eof => StreamEvent::Ended,
            ReadResult::Line(line) => StreamEvent::Line(line),
            ReadResult::Batch(lines) => StreamEvent::Lines(lines),
        })
    }
}
