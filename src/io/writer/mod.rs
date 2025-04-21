pub mod stdout;
pub mod temp_file;

use crate::io::controller::Writer;
use async_trait::async_trait;
use miette::Result;

#[async_trait]
pub trait AsyncLineWriter {
    async fn write(&mut self, line: &str) -> Result<()>;
}

#[async_trait]
impl AsyncLineWriter for Writer {
    async fn write(&mut self, line: &str) -> Result<()> {
        match self {
            Writer::TempFile(w) => w.write(line).await,
            Writer::Stdout(w) => w.write(line).await,
        }
    }
}
