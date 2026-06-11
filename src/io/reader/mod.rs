pub mod command;
pub mod file_reader;
mod line_batcher;
pub mod stdin;

use crate::io::reader::command::CommandReader;
use crate::io::reader::file_reader::FileReader;
use crate::io::reader::stdin::StdinReader;
use anyhow::Result;

pub enum Reader {
    File(FileReader),
    Stdin(StdinReader),
    Command(CommandReader),
}

#[derive(Debug)]
pub enum StreamEvent {
    Started,
    Ended,
    Line(String),
    Lines(Vec<String>),
}

impl Reader {
    pub async fn next(&mut self) -> Result<StreamEvent> {
        match self {
            Reader::File(r) => r.next().await,
            Reader::Stdin(r) => r.next().await,
            Reader::Command(r) => r.next().await,
        }
    }
}
