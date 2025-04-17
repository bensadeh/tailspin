use crate::initial_read::InitialReadCompleteSender;
use crate::io::controller::Reader;
use crate::io::reader::common::{BUFF_READER_CAPACITY, ReadResult, read_lines};
use crate::io::reader::{AsyncLineReader, ReadType};
use async_trait::async_trait;
use miette::Result;
use tokio::io::{BufReader, Stdin};

pub struct StdinReader {
    reader: BufReader<Stdin>,
    irc_sender: InitialReadCompleteSender,
}

impl StdinReader {
    pub fn get_reader(irc_sender: InitialReadCompleteSender) -> Reader {
        let reader = BufReader::with_capacity(BUFF_READER_CAPACITY, tokio::io::stdin());
        let stdin_reader = StdinReader { reader, irc_sender };

        Reader::Stdin(stdin_reader)
    }
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next(&mut self) -> Result<ReadType> {
        match read_lines(&mut self.reader).await? {
            ReadResult::Eof => {
                self.irc_sender.send()?;

                Ok(ReadType::StreamEnded)
            }
            ReadResult::Line(line) => Ok(ReadType::SingleLine(line)),
            ReadResult::Batch(lines) => Ok(ReadType::MultipleLines(lines)),
        }
    }
}
