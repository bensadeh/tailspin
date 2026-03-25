use crate::io::writer::AsyncLineWriter;
use anyhow::{Context, Result};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct TempFile {
    writer: BufWriter<File>,
}

impl TempFile {
    #[allow(clippy::unused_async)]
    pub async fn new(writer: BufWriter<File>) -> Self {
        TempFile { writer }
    }
}

impl AsyncLineWriter for TempFile {
    async fn write(&mut self, line: &str) -> Result<()> {
        self.writer
            .write_all(line.as_bytes())
            .await
            .context("Failed to write line to file")?;

        self.writer
            .write_all(b"\n")
            .await
            .context("Failed to write line to file")?;

        // Flush after each write so the pager (e.g. less +F) sees lines immediately
        self.writer.flush().await.context("Error flushing temp file")?;

        Ok(())
    }
}
