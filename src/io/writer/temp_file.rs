use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct TempFile {
    writer: BufWriter<File>,
}

impl TempFile {
    pub async fn new(writer: BufWriter<File>) -> Self {
        TempFile { writer }
    }
}

#[async_trait]
impl AsyncLineWriter for TempFile {
    async fn write(&mut self, line: &str) -> Result<()> {
        let line_with_newline = format!("{}\n", line);

        self.writer
            .write_all(line_with_newline.as_bytes())
            .await
            .into_diagnostic()
            .wrap_err("Failed to write line to file")?;

        self.writer
            .flush()
            .await
            .into_diagnostic()
            .wrap_err("Error flushing temp file")?;

        Ok(())
    }
}
