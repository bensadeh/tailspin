use crate::io::controller::Presenter;
use miette::Result;

pub mod custom_pager;
pub mod less;
pub mod stdout;

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
            Presenter::Less(p) => p.present().await,
            Presenter::CustomPager(p) => p.present().await,
            Presenter::StdOut(p) => p.present().await,
        }
    }
}
