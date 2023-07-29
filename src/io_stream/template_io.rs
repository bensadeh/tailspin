use crate::io_stream::linemux_reader::LinemuxReader;
use crate::io_stream::traits::{AsyncLineReader, AsyncLineWriter};
use crate::io_stream::LineIOStream;
use crate::types::{Input, Output};
use async_trait::async_trait;
use rand::random;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io;
use tokio::io::BufWriter;
use tokio::sync::oneshot::Sender;

pub struct TemplateIOStream {
    reader: Box<dyn AsyncLineReader + Send>,
}

impl TemplateIOStream {
    pub async fn new(
        file_path: String,
        number_of_lines: usize,
        reached_eof_tx: Option<Sender<()>>,
    ) -> Self {
        let reader = LinemuxReader::new(file_path, number_of_lines, reached_eof_tx)
            .await
            .unwrap();
        Self {
            reader: Box::new(reader),
        }
    }
}

#[async_trait]
impl LineIOStream for TemplateIOStream {
    async fn next_line(&mut self) -> io::Result<Option<&str>> {
        // self.reader.next_line().await
        unimplemented!()
    }

    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        // self.writer.write_line(line).await
        unimplemented!()
    }
}

async fn create_temp_file() -> (tempfile::TempDir, PathBuf, BufWriter<File>) {
    let unique_id: u32 = random();
    let filename = format!("tailspin.temp.{}", unique_id);

    let temp_dir = tempfile::tempdir().unwrap();

    let output_path = temp_dir.path().join(filename);
    let output_file = File::create(&output_path).await.unwrap();
    let output_writer = BufWriter::new(output_file);

    (temp_dir, output_path, output_writer)
}
