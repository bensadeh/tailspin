use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use owo_colors::OwoColorize;
use rand::random;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct TempFile {
    pub path: PathBuf,
    _temp_dir: TempDir,
    file_writer: BufWriter<File>,
}

impl TempFile {
    pub async fn new() -> Self {
        let (_temp_dir, temp_file_path, temp_file_writer) = create_temp_file().await;

        TempFile {
            _temp_dir,
            path: temp_file_path,
            file_writer: temp_file_writer,
        }
    }
}

#[async_trait]
impl AsyncLineWriter for TempFile {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        let line_with_newline = format!("{}\n", line);

        let write_result = self.file_writer.write_all(line_with_newline.as_bytes()).await;
        if let Err(e) = write_result {
            println!("Error writing to temp file: {}", e.yellow());
        }

        let flush_result = self.file_writer.flush().await;
        if let Err(e) = flush_result {
            println!("Error flushing temp file: {}", e.yellow());
        }

        Ok(())
    }
}

async fn create_temp_file() -> (TempDir, PathBuf, BufWriter<File>) {
    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{}", unique_id);

    let temp_dir = tempfile::tempdir().unwrap();

    let temp_file_path = temp_dir.path().join(filename);
    let output_file = File::create(&temp_file_path).await.unwrap();
    let output_writer = BufWriter::new(output_file);

    (temp_dir, temp_file_path, output_writer)
}
