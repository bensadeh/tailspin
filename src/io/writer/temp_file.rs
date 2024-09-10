use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use color_eyre::owo_colors::OwoColorize;
use rand::random;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct TempFile {
    _temp_dir: TempDir,
    temp_file_writer: BufWriter<File>,
}

pub struct TempFileWriterResult {
    pub writer: Box<dyn AsyncLineWriter + Send>,
    pub temp_file_path: String,
}

impl TempFile {
    pub async fn get_writer_result() -> TempFileWriterResult {
        let (temp_dir, temp_file_path, temp_file_writer) = create_temp_file().await;

        let temp_file_path_string = temp_file_path
            .to_str()
            .expect("Could not get path to temp file")
            .to_owned();

        TempFileWriterResult {
            writer: Box::new(TempFile {
                _temp_dir: temp_dir,
                temp_file_writer,
            }),
            temp_file_path: temp_file_path_string,
        }
    }
}

#[async_trait]
impl AsyncLineWriter for TempFile {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        let line_with_newline = format!("{line}\n");

        let write_result = self.temp_file_writer.write_all(line_with_newline.as_bytes()).await;
        if let Err(e) = write_result {
            println!("Error writing to temp file: {}", e.yellow());
        }

        let flush_result = self.temp_file_writer.flush().await;
        if let Err(e) = flush_result {
            println!("Error flushing temp file: {}", e.yellow());
        }

        Ok(())
    }
}

async fn create_temp_file() -> (TempDir, PathBuf, BufWriter<File>) {
    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{unique_id}");

    let temp_dir = tempfile::tempdir().unwrap();

    let temp_file_path = temp_dir.path().join(filename);
    let output_file = File::create(&temp_file_path).await.unwrap();
    let output_writer = BufWriter::new(output_file);

    (temp_dir, temp_file_path, output_writer)
}
