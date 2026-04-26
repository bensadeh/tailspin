pub mod stdout;
pub mod temp_file;

use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;
use anyhow::Result;

pub enum Writer {
    TempFile(TempFile),
    Stdout(StdoutWriter),
}

pub trait AsyncLineWriter {
    async fn write(&mut self, line: &str) -> Result<()>;
}

impl AsyncLineWriter for Writer {
    async fn write(&mut self, line: &str) -> Result<()> {
        match self {
            Writer::TempFile(w) => w.write(line).await,
            Writer::Stdout(w) => w.write(line).await,
        }
    }
}
