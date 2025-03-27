use crate::io::controller::PresenterImpl;
use miette::Result;

pub mod custom_pager;
pub mod empty;
pub mod less;

pub trait Present: Send {
    fn present(&self) -> Result<()>;
}

impl Present for PresenterImpl {
    fn present(&self) -> Result<()> {
        match self {
            PresenterImpl::Less(p) => p.present(),
            PresenterImpl::CustomPager(p) => p.present(),
            PresenterImpl::NoPresenter(p) => p.present(),
        }
    }
}
