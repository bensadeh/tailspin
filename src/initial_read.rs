use miette::{Diagnostic, Result};
use thiserror::Error;
use tokio::sync::oneshot;

pub fn initial_read_complete_channel() -> (InitialReadCompleteSender, InitialReadCompleteReceiver) {
    let (tx, rx) = oneshot::channel();
    (InitialReadCompleteSender::new(tx), InitialReadCompleteReceiver::new(rx))
}

#[derive(Debug)]
pub struct InitialReadCompleteReceiver(oneshot::Receiver<()>);

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to receive initial-read-complete signal")]
#[diagnostic(help("Ensure the IO task completes correctly and signals initial read completion."))]
pub struct InitialReadCompleteRecvError(#[source] oneshot::error::RecvError);

impl InitialReadCompleteReceiver {
    pub const fn new(receiver: oneshot::Receiver<()>) -> Self {
        InitialReadCompleteReceiver(receiver)
    }

    pub async fn wait(self) -> Result<()> {
        self.0.await.map_err(InitialReadCompleteRecvError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct InitialReadCompleteSender(Option<oneshot::Sender<()>>);

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send initial-read-complete signal")]
#[diagnostic(help("The receiver was dropped early. Ensure it remains alive until initial read completes."))]
pub struct InitialReadCompleteSendError;

impl InitialReadCompleteSender {
    pub const fn new(sender: oneshot::Sender<()>) -> Self {
        InitialReadCompleteSender(Some(sender))
    }

    pub fn send(&mut self) -> Result<()> {
        match self.0.take() {
            Some(sender) => Ok(sender.send(()).map_err(|_| InitialReadCompleteSendError)?),
            None => Ok(()),
        }
    }
}
