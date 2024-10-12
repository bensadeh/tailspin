pub mod command;
pub mod linemux;
pub mod stdin;

use async_trait::async_trait;
use tokio::io;

#[async_trait]
pub trait AsyncLineReader {
    async fn next_line_batch(&mut self) -> io::Result<Option<Vec<String>>>;
}
