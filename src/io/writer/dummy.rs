use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use tokio::io;

pub struct NoWriter {}

impl NoWriter {
    pub fn init() -> Box<dyn AsyncLineWriter + Send> {
        Box::new(NoWriter {})
    }
}

#[async_trait]
impl AsyncLineWriter for NoWriter {
    async fn write_line(&mut self, _line: &str) -> io::Result<()> {
        //no-op
        Ok(())
    }
}
