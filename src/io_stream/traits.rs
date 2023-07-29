use async_trait::async_trait;
use tokio::io;

#[async_trait]
pub trait LineIOStream: Send {
    async fn next_line(&mut self) -> io::Result<Option<&str>>;
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}

#[async_trait]
pub trait AsyncLineReader {
    async fn next_line(&mut self) -> io::Result<Option<&str>>;
}

#[async_trait]
pub trait AsyncLineWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}
