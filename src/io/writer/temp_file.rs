use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct TempFile {
    writer: BufWriter<File>,
}

impl TempFile {
    pub const fn new(writer: BufWriter<File>) -> Self {
        TempFile { writer }
    }
}

impl TempFile {
    pub fn write_batch<'a>(&mut self, lines: impl Iterator<Item = &'a str>) -> Result<()> {
        for line in lines {
            self.writer
                .write_all(line.as_bytes())
                .context("Failed to write line to file")?;

            self.writer.write_all(b"\n").context("Failed to write line to file")?;
        }

        // Flush after each batch so the pager (e.g. less +F) sees lines immediately
        self.writer.flush().context("Error flushing temp file")?;

        Ok(())
    }
}
