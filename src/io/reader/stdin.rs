use crate::io::reader::StreamEvent;
use crate::io::reader::line_batcher::{BUF_READER_CAPACITY, ReadResult, read_lines};
use anyhow::Result;
use tokio::io::{BufReader, Stdin, stdin};

pub struct StdinReader {
    reader: BufReader<Stdin>,
    initial_read_complete_sent: bool,
}

impl StdinReader {
    pub fn new() -> StdinReader {
        let reader = BufReader::with_capacity(BUF_READER_CAPACITY, stdin());

        StdinReader {
            reader,
            initial_read_complete_sent: false,
        }
    }
}

impl StdinReader {
    pub async fn next(&mut self) -> Result<StreamEvent> {
        if !self.initial_read_complete_sent {
            self.initial_read_complete_sent = true;

            return Ok(StreamEvent::InitialReadComplete);
        }

        match read_lines(&mut self.reader).await? {
            ReadResult::Eof => Ok(StreamEvent::Ended),
            ReadResult::Line(line) => Ok(StreamEvent::Line(line)),
            ReadResult::Batch(lines) => Ok(StreamEvent::Lines(lines)),
        }
    }
}
