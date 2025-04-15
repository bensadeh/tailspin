use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::utils::read_available_lines;
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result, miette};
use std::process::Stdio;
use tokio::io::BufReader;
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
}

#[async_trait]
impl AsyncLineReader for CommandReader {
    async fn next(&mut self) -> Result<ReadType> {
        let buffer = read_available_lines(&mut self.reader).await?;

        if buffer.is_empty() {
            return Ok(ReadType::StreamEnded);
        }

        Ok(ReadType::MultipleLines(buffer))
    }
}
