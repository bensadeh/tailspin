use async_trait::async_trait;
use tokio::io;

pub struct IoStream<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> IoStream<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }

    pub async fn next_line(&mut self) -> io::Result<Option<String>>
    where
        R: AsyncLineReader + Unpin,
    {
        self.reader.next_line().await
    }

    pub async fn write_line(&mut self, line: &str) -> io::Result<()>
    where
        W: AsyncLineWriter + Unpin,
    {
        self.writer.write_line(line).await
    }
}

#[async_trait]
pub trait AsyncLineReader {
    async fn next_line(&mut self) -> io::Result<Option<String>>;
}

#[async_trait]
pub trait AsyncLineWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}
