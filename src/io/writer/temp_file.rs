use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use miette::{Context, IntoDiagnostic, Result};
use rand::random;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct TempFile {
    pub path: PathBuf,
    _temp_dir: TempDir,
    file_writer: BufWriter<File>,
}

impl TempFile {
    pub async fn new() -> Result<Self> {
        let (_temp_dir, temp_file_path, temp_file_writer) = create_temp_file().await?;

        Ok(TempFile {
            _temp_dir,
            path: temp_file_path,
            file_writer: temp_file_writer,
        })
    }
}

#[async_trait]
impl AsyncLineWriter for TempFile {
    async fn write_line(&mut self, line: &str) -> Result<()> {
        let line_with_newline = format!("{}\n", line);

        self.file_writer
            .write_all(line_with_newline.as_bytes())
            .await
            .into_diagnostic()
            .wrap_err("Failed to write line to file")?;

        self.file_writer
            .flush()
            .await
            .into_diagnostic()
            .wrap_err("Error flushing temp file")?;

        Ok(())
    }
}

async fn create_temp_file() -> Result<(TempDir, PathBuf, BufWriter<File>)> {
    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{}", unique_id);

    let temp_dir = tempfile::tempdir()
        .into_diagnostic()
        .wrap_err("Could not locate temporary directory")?;

    let temp_file_path = temp_dir.path().join(filename);
    let output_file = File::create(&temp_file_path)
        .await
        .into_diagnostic()
        .wrap_err("Could not create temporary file")?;

    let output_writer = BufWriter::new(output_file);

    Ok((temp_dir, temp_file_path, output_writer))
}
