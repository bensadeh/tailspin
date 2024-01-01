pub mod minus;
pub mod stdout;

use async_trait::async_trait;
use tokio::io;

#[async_trait]
pub trait AsyncLineWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()>;
}
