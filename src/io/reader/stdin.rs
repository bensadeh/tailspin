use crate::io::reader::common::{BUFF_READER_CAPACITY, ReadResult, read_lines};
use crate::io::reader::{AsyncLineReader, StreamEvent};
use async_trait::async_trait;
use miette::Result;
use tokio::io::{BufReader, Stdin, stdin};

pub struct StdinReader {
    reader: BufReader<Stdin>,
    stream_started: bool,
}

impl StdinReader {
    pub fn new() -> StdinReader {
        let reader = BufReader::with_capacity(BUFF_READER_CAPACITY, stdin());
        let stream_started = false;

        StdinReader { reader, stream_started }
    }
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next(&mut self) -> Result<StreamEvent> {
        if !self.stream_started {
            self.stream_started = true;

            return Ok(StreamEvent::Started);
        }

        match read_lines(&mut self.reader).await? {
            ReadResult::Eof => Ok(StreamEvent::Ended),
            ReadResult::Line(line) => Ok(StreamEvent::Line(line)),
            ReadResult::Batch(lines) => Ok(StreamEvent::Lines(lines)),
        }
    }
}
