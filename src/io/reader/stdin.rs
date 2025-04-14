use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::utils::read_complete_lines;
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use miette::Result;
use tokio::io::{BufReader, Stdin};

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
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next(&mut self) -> Result<ReadType> {
        let buffer = read_complete_lines(&mut self.reader).await?;

        if buffer.is_empty() {
            self.initial_read_complete_sender.send()?;

            return Ok(ReadType::StreamEnded);
        }

        Ok(ReadType::MultipleLines(buffer))
    }
}
