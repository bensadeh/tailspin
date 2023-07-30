use crate::reader::AsyncLineReader;
use async_trait::async_trait;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader, Lines, Stdin};
use tokio::sync::oneshot::Sender;

pub struct StdinReader {
    reader: Lines<BufReader<Stdin>>,
    reached_eof_tx: Option<Sender<()>>,
}

impl StdinReader {
    pub fn get_reader(reached_eof_tx: Option<Sender<()>>) -> Box<dyn AsyncLineReader + Send> {
        Box::new(StdinReader {
            reader: BufReader::new(tokio::io::stdin()).lines(),
            reached_eof_tx,
        })
    }
}

#[async_trait]
impl AsyncLineReader for StdinReader {
    async fn next_line(&mut self) -> io::Result<Option<String>> {
        match self.reader.next_line().await {
            Ok(Some(line)) => Ok(Some(line)),
            Ok(None) => {
                if let Some(reached_eof) = self.reached_eof_tx.take() {
                    reached_eof
                        .send(())
                        .expect("Failed sending EOF signal to oneshot channel");
                }
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}
