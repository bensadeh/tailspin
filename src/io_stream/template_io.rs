use crate::io_stream::linemux_reader::LinemuxReader;
use crate::io_stream::temp_file_writer::TempFileWriter;
use crate::io_stream::traits::{AsyncLineReader, AsyncLineWriter};
use crate::io_stream::LineIOStream;
use async_trait::async_trait;
use tokio::io;
use tokio::sync::oneshot::Sender;

pub struct TemplateIOStream {
    reader: Box<dyn AsyncLineReader + Send>,
    writer: Box<dyn AsyncLineWriter + Send>,
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
        let writer = TempFileWriter::new().await;

        Self {
            reader: Box::new(reader),
            writer: Box::new(writer),
        }
    }
}

#[async_trait]
impl LineIOStream for TemplateIOStream {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        self.reader.next_line().await
    }

    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        self.writer.write_line(line).await
    }
}
