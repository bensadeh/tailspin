use crate::writer::AsyncLineWriter;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncWriteExt, BufWriter, Stdout};

pub struct StdoutWriter {
    writer: BufWriter<Stdout>,
}

impl StdoutWriter {
    pub fn new() -> Box<dyn AsyncLineWriter + Send> {
        Box::new(StdoutWriter {
            writer: BufWriter::new(tokio::io::stdout()),
        })
    }
}

#[async_trait]
impl AsyncLineWriter for StdoutWriter {
    async fn write_line(&mut self, line: &str) -> io::Result<()> {
        let line_with_newline = format!("{}\n", line);

        self.writer.write_all(line_with_newline.as_bytes()).await?;

        self.writer.flush().await?;

        Ok(())
    }
}
