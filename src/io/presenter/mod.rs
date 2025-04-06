use crate::io::controller::Presenter;
use miette::Result;

pub mod custom_pager;
pub mod empty;
pub mod less;

pub trait Present: Send {
    fn present(&self) -> Result<()>;
}

impl Present for Presenter {
    fn present(&self) -> Result<()> {
        match self {
            Presenter::Less(p) => p.present(),
            Presenter::CustomPager(p) => p.present(),
            Presenter::None(p) => p.present(),
        }
    }
}
