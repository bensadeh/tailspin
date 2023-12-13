pub mod command;
pub mod linemux;
pub mod stdin;

use async_trait::async_trait;
use tokio::{io, sync::oneshot::Sender};
/// A helper type, if set, can be used to throw a signal
// / that the EOF has been reached by the reader.
pub type EOFSignaler = Option<Sender<()>>;

#[async_trait]
pub trait AsyncLineReader {
    async fn next_line(&mut self) -> io::Result<Option<String>>;
}

fn send_eof_signal(eof_signaler: EOFSignaler) {
    if let Some(eof_signaler) = eof_signaler {
        eof_signaler
            .send(())
            .expect("Failed sending EOF signal to oneshot channel");
    }
}
