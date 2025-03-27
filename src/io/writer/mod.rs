pub mod stdout;
pub mod temp_file;

use crate::io::controller::Writer;
use async_trait::async_trait;
use tokio::io;

#[async_trait]
pub trait AsyncLineWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}

#[async_trait]
impl AsyncLineWriter for Writer {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        match self {
            Writer::TempFile(w) => w.write_line(line).await,
            Writer::Stdout(w) => w.write_line(line).await,
        }
    }
}
