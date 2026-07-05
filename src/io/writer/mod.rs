pub mod stdout;
pub mod temp_file;

use crate::io::writer::stdout::StdoutWriter;
use crate::io::writer::temp_file::TempFile;
use anyhow::Result;

pub enum Writer {
    TempFile(TempFile),
    Stdout(StdoutWriter),
}

impl Writer {
    pub fn write_batch<'a>(&mut self, lines: impl Iterator<Item = &'a str>) -> Result<()> {
        match self {
            Writer::TempFile(w) => w.write_batch(lines),
            Writer::Stdout(w) => w.write_batch(lines),
        }
    }
}
