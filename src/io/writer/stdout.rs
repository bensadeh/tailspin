use anyhow::Result;
use std::io::{self, BufWriter, Write as _};
use thiserror::Error;

const WRITE_BUFFER_CAPACITY: usize = 256 * 1024;

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

// Holds the `Stdout` handle rather than its lock: the writer moves into the
// stream thread, and `StdoutLock` is not `Send`.
pub struct StdoutWriter {
    inner: BufWriter<io::Stdout>,
}

impl StdoutWriter {
    pub fn new() -> Self {
        Self {
            inner: BufWriter::with_capacity(WRITE_BUFFER_CAPACITY, io::stdout()),
        }
    }

    /// Writes each line followed by `\n`, flushing once at the end so the
    /// batch becomes visible immediately.
    pub fn write_batch<'a>(&mut self, lines: impl Iterator<Item = &'a str>) -> Result<()> {
        match self.write_and_flush(lines) {
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Err(BrokenPipe)?,
            result => Ok(result?),
        }
    }

    fn write_and_flush<'a>(&mut self, lines: impl Iterator<Item = &'a str>) -> io::Result<()> {
        for line in lines {
            self.inner.write_all(line.as_bytes())?;
            self.inner.write_all(b"\n")?;
        }
        self.inner.flush()
    }
}
