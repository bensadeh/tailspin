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

pub fn write_line(line: &str) -> Result<()> {
    match writeln!(io::stdout(), "{line}") {
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Err(BrokenPipe)?,
        result => Ok(result?),
    }
}
