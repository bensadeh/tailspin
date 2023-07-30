use crate::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};
use tokio::sync::oneshot::Sender;

pub struct StdinReader {
    reader: BufReader<Stdin>,
}

impl StdinReader {
    pub fn get_reader(reached_eof_tx: Option<Sender<()>>) -> Box<dyn AsyncLineReader + Send> {
        if let Some(reached_eof) = reached_eof_tx {
            reached_eof
                .send(())
                .expect("Failed sending EOF signal to oneshot channel");
        }

        Box::new(StdinReader {
            reader: BufReader::new(tokio::io::stdin()),
        })
    }
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        let mut buf = String::new();
        let bytes_read = self.reader.read_line(&mut buf).await?;
        if bytes_read == 0 {
            Ok(None)
        } else {
            Ok(Some(buf))
        }
    }
}
