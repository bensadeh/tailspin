use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::utils::{ReadResult, read_lines};
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
        match read_lines(&mut self.reader).await? {
            ReadResult::Eof => {
                self.initial_read_complete_sender.send()?;

                Ok(ReadType::StreamEnded)
            }
            ReadResult::Line(line) => Ok(ReadType::SingleLine(line)),
            ReadResult::Batch(lines) => Ok(ReadType::MultipleLines(lines)),
        }
    }
}
