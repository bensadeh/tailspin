use crate::io::presenter::Present;
use miette::Result;
use std::future::pending;

/// `StdoutPresenter` does not require any special presentation logic because
/// the output is already directly handled by a dedicated stdout writer.
/// Writing to stdout is sufficient, eliminating the need for additional
/// presentation mechanisms.
pub struct StdoutPresenter {
    _private: (),
}

impl StdoutPresenter {
    pub const fn new() -> StdoutPresenter {
        Self { _private: () }
    }
}

impl Present for StdoutPresenter {
    async fn present(&self) -> Result<()> {
        pending::<()>().await;

        Ok(())
    }
}
