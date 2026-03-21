use anyhow::Result;
use thiserror::Error;
use tokio::sync::oneshot;

pub fn initial_read_complete_channel() -> (InitialReadCompleteSender, InitialReadCompleteReceiver) {
    let (tx, rx) = oneshot::channel();
    (InitialReadCompleteSender(Some(tx)), InitialReadCompleteReceiver(rx))
}

#[derive(Debug, Error)]
enum SignalError {
    #[error("Failed to receive initial-read-complete signal")]
    Recv(#[source] oneshot::error::RecvError),
    #[error("Failed to send initial-read-complete signal")]
    Send,
}

#[derive(Debug)]
pub struct InitialReadCompleteReceiver(oneshot::Receiver<()>);

impl InitialReadCompleteReceiver {
    pub async fn receive(self) -> Result<()> {
        self.0.await.map_err(SignalError::Recv)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct InitialReadCompleteSender(Option<oneshot::Sender<()>>);

impl InitialReadCompleteSender {
    pub fn send(&mut self) -> Result<()> {
        match self.0.take() {
            Some(sender) => Ok(sender.send(()).map_err(|_| SignalError::Send)?),
            None => Ok(()),
        }
    }
}
