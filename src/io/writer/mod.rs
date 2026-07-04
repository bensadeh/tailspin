pub mod stdout;
pub mod temp_file;

use crate::io::writer::temp_file::TempFile;
use anyhow::Result;

pub enum Writer {
    TempFile(TempFile),
    Stdout,
}

impl Writer {
    pub fn write(&mut self, line: &str) -> Result<()> {
        match self {
            Writer::TempFile(w) => w.write(line),
            Writer::Stdout => stdout::write_line(line),
        }
    }
}
