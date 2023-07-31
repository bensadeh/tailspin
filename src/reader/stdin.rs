use crate::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{stdin, AsyncBufReadExt, BufReader, Stdin};
use tokio::sync::oneshot::Sender;

pub struct StdinReader {
    reader: BufReader<Stdin>,
    reached_eof_tx: Option<Sender<()>>,
}

impl StdinReader {
    pub fn get_reader(reached_eof_tx: Option<Sender<()>>) -> Box<dyn AsyncLineReader + Send> {
        Box::new(StdinReader {
            reader: BufReader::new(stdin()),
            reached_eof_tx,
        })
    }
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        let mut buffer = Vec::new();
        match self.reader.read_until(b'\n', &mut buffer).await {
            Ok(0) => {
                if let Some(reached_eof) = self.reached_eof_tx.take() {
                    reached_eof
                        .send(())
                        .expect("Failed sending EOF signal to oneshot channel");
                }
                Ok(None)
            }
            Ok(_) => {
                if let Some(last_byte) = buffer.last() {
                    if *last_byte == b'\n' {
                        buffer.pop();
                    }
                }
                let line = String::from_utf8_lossy(&buffer);
                Ok(Some(line.into_owned()))
            }
            Err(e) => Err(e),
        }
    }
}
