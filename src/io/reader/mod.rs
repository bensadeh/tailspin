mod buffer_line_counter;
pub mod command;
mod file_line_counter;
pub mod linemux;
pub mod stdin;

use crate::io::controller::Reader;
use async_trait::async_trait;
use miette::Result;

#[derive(Debug)]
pub enum StreamEvent {
    Started,
    Ended,
    Line(String),
    Lines(Vec<String>),
}

#[async_trait]
pub trait AsyncLineReader {
    async fn next(&mut self) -> Result<StreamEvent>;
}

#[async_trait]
impl AsyncLineReader for Reader {
    async fn next(&mut self) -> Result<StreamEvent> {
        match self {
            Reader::Linemux(r) => r.next().await,
            Reader::Stdin(r) => r.next().await,
            Reader::Command(r) => r.next().await,
        }
    }
}
