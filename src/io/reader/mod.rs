pub mod command;
pub mod linemux;
pub mod stdin;

use crate::io::controller::Reader;
use async_trait::async_trait;
use miette::{Diagnostic, Result};
use thiserror::Error;
use tokio::io;

#[async_trait]
pub trait AsyncLineReader {
    async fn next_line_batch(&mut self) -> Result<Option<Vec<String>>>;
}

#[async_trait]
impl AsyncLineReader for Reader {
    async fn next_line_batch(&mut self) -> Result<Option<Vec<String>>> {
        match self {
            Reader::Linemux(r) => r.next_line_batch().await,
            Reader::Stdin(r) => r.next_line_batch().await,
            Reader::Command(r) => r.next_line_batch().await,
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum ReaderError {
    #[error("Error reading stream")]
    IoError(#[source] io::Error),
}
