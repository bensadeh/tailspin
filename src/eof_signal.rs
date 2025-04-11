use miette::Diagnostic;
use thiserror::Error;
use tokio::sync::oneshot;

pub fn eof_signal_channel() -> (EofSignalSender, EofSignalReceiver) {
    let (tx, rx) = oneshot::channel();
    (EofSignalSender::new(tx), EofSignalReceiver::new(rx))
}

#[derive(Debug)]
pub struct EofSignalReceiver(oneshot::Receiver<()>);

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to receive End Of File (EOF) signal")]
#[diagnostic(help("Ensure the IO task completes correctly and sends the EOF signal."))]
pub struct EofSignalRecvError(#[source] oneshot::error::RecvError);

impl EofSignalReceiver {
    pub const fn new(receiver: oneshot::Receiver<()>) -> Self {
        EofSignalReceiver(receiver)
    }

    pub async fn wait(self) -> Result<(), EofSignalRecvError> {
        self.0.await.map_err(EofSignalRecvError)
    }
}

#[derive(Debug)]
pub struct EofSignalSender(Option<oneshot::Sender<()>>);

#[derive(Debug, Diagnostic, Error)]
#[error("Failed to send End Of File (EOF) signal")]
#[diagnostic(help("The EOF receiver was dropped early. Ensure the receiver remains alive until EOF is signaled."))]
pub struct EofSignalSendError;

impl EofSignalSender {
    pub const fn new(sender: oneshot::Sender<()>) -> Self {
        EofSignalSender(Some(sender))
    }

    pub fn send(&mut self) -> Result<(), EofSignalSendError> {
        match self.0.take() {
            Some(sender) => sender.send(()).map_err(|_| EofSignalSendError),
            None => Ok(()),
        }
    }
}
