use crate::io::reader::StreamEvent;
use crate::io::reader::line_batcher::{BUF_READER_CAPACITY, ReadResult, read_lines};
use anyhow::Result;
use tokio::io::{BufReader, Stdin, stdin};

pub struct StdinReader {
    reader: BufReader<Stdin>,
    stream_started: bool,
}

impl StdinReader {
    pub fn new() -> StdinReader {
        let reader = BufReader::with_capacity(BUF_READER_CAPACITY, stdin());
        let stream_started = false;

        StdinReader { reader, stream_started }
    }
}

impl StdinReader {
    pub async fn next(&mut self) -> Result<StreamEvent> {
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
