pub mod dummy;
pub mod stdout;
pub mod temp_file;

use async_trait::async_trait;
use tokio::io;

#[async_trait]
pub trait AsyncLineWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}
