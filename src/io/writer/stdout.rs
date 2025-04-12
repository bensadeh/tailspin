use crate::io::controller::Writer;
use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use miette::Result;

pub struct StdoutWriter {}

impl StdoutWriter {
    pub const fn init() -> Writer {
        Writer::Stdout(StdoutWriter {})
    }
}

#[async_trait]
impl AsyncLineWriter for StdoutWriter {
    async fn write_line(&mut self, line: &str) -> Result<()> {
        println!("{}", line);

        Ok(())
    }
}
