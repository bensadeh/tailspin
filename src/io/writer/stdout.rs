use crate::io::writer::AsyncLineWriter;
use anyhow::Result;
use std::io::{self, Write as _};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("broken pipe")]
pub struct BrokenPipe;

impl BrokenPipe {
    pub fn suppress(result: Result<()>) -> Result<()> {
        match result {
            Err(report) if report.downcast_ref::<Self>().is_some() => Ok(()),
            other => other,
        }
    }
}

pub struct StdoutWriter {
    _private: (),
}

impl StdoutWriter {
    pub const fn new() -> StdoutWriter {
        StdoutWriter { _private: () }
    }
}

impl AsyncLineWriter for StdoutWriter {
    async fn write(&mut self, line: &str) -> Result<()> {
        match writeln!(io::stdout(), "{}", line) {
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Err(BrokenPipe)?,
            result => Ok(result?),
        }
    }
}
