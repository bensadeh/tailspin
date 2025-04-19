use crate::io::controller::Reader;
use crate::io::reader::common::{BUFF_READER_CAPACITY, ReadResult, read_lines};
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::process::Stdio;
use tokio::io::BufReader;
use tokio::process::Command as AsyncCommand;

pub struct CommandReader {
    reader: BufReader<tokio::process::ChildStdout>,
    ready: bool,
}

impl CommandReader {
    pub async fn get_reader(command: String) -> Result<Reader> {
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

        let reader = BufReader::with_capacity(BUFF_READER_CAPACITY, stdout);

        Ok(Reader::Command(CommandReader { reader, ready: false }))
    }
}

#[async_trait]
impl AsyncLineReader for CommandReader {
    async fn next(&mut self) -> Result<ReadType> {
        if !self.ready {
            self.ready = !self.ready;

            return Ok(ReadType::StreamStarted);
        }

        read_lines(&mut self.reader).await.map(|res| match res {
            ReadResult::Eof => ReadType::StreamEnded,
            ReadResult::Line(line) => ReadType::SingleLine(line),
            ReadResult::Batch(lines) => ReadType::MultipleLines(lines),
        })
    }
}
