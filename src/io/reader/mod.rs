pub mod command;
mod common;
pub mod linemux;
pub mod stdin;

use crate::io::controller::Reader;
use async_trait::async_trait;
use miette::Result;

pub enum ReadType {
    StreamEnded,
    StreamStarted,
    SingleLine(String),
    MultipleLines(Vec<String>),
    InitialRead(Vec<String>),
}

#[async_trait]
pub trait AsyncLineReader {
    async fn next(&mut self) -> Result<ReadType>;
}

#[async_trait]
impl AsyncLineReader for Reader {
    async fn next(&mut self) -> Result<ReadType> {
        match self {
            Reader::Linemux(r) => r.next().await,
            Reader::Stdin(r) => r.next().await,
            Reader::Command(r) => r.next().await,
        }
    }
}
