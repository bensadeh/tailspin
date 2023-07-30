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

pub struct Foo {
    pub bar: i32,
}

impl Foo {
    async fn new() -> Self {
        Foo { bar: 42 }
    }

    pub fn bar(&self) -> i32 {
        self.bar
    }
}

impl TemplateIOStream {
    pub(crate) async fn new(
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

pub async fn create_stream_and_foo(
    file_path: String,
    number_of_lines: usize,
    reached_eof_tx: Option<Sender<()>>,
) -> (TemplateIOStream, Foo) {
    let stream = TemplateIOStream::new(file_path, number_of_lines, reached_eof_tx).await;
    let foo = Foo::new().await;

    (stream, foo)
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
