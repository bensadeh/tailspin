use crate::io::writer::AsyncLineWriter;
use async_trait::async_trait;
use miette::Result;

pub struct StdoutWriter {
    _private: (),
}

impl StdoutWriter {
    pub const fn new() -> StdoutWriter {
        StdoutWriter { _private: () }
    }
}

#[async_trait]
impl AsyncLineWriter for StdoutWriter {
    async fn write(&mut self, line: &str) -> Result<()> {
        println!("{}", line);

        Ok(())
    }
}
