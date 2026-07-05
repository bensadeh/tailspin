use crate::io::reader::StreamEvent;
use crate::io::reader::line_batcher::{BUF_READER_CAPACITY, ReadResult, read_batch};
use anyhow::Result;
use std::io::{BufReader, Stdin, stdin};

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

    pub fn next(&mut self) -> Result<StreamEvent> {
        if !self.initial_read_complete_sent {
            self.initial_read_complete_sent = true;

            return Ok(StreamEvent::InitialReadComplete);
        }

        match read_batch(&mut self.reader)? {
            ReadResult::Eof => Ok(StreamEvent::Ended),
            ReadResult::Batch(batch) => Ok(StreamEvent::Lines(batch)),
        }
    }
}
