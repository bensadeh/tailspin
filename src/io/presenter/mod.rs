pub mod pager;
pub mod stdout;

use crate::io::presenter::pager::Pager;
use crate::io::presenter::stdout::StdoutPresenter;
use anyhow::Result;

pub enum Presenter {
    Pager(Pager),
    StdOut(StdoutPresenter),
}

/// Presenters are responsible for displaying output to the user.
/// Different implementations handle output differently—e.g., direct stdout,
/// paging via `less`, or using a custom pager.
///
/// When `present()` returns, the application terminates. For continuous
/// output scenarios, implementations should ensure they never return.
pub trait Present: Send {
    async fn present(&self) -> Result<()>;
}

impl Present for Presenter {
    async fn present(&self) -> Result<()> {
        match self {
            Presenter::Pager(p) => p.present().await,
            Presenter::StdOut(p) => p.present().await,
        }
    }
}
