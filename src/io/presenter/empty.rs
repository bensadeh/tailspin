use crate::io::controller::PresenterImpl;
use crate::io::presenter::Present;
use miette::Result;

pub struct NoPresenter {}

impl NoPresenter {
    pub const fn get_presenter() -> PresenterImpl {
        PresenterImpl::NoPresenter(Self {})
    }
}

impl Present for NoPresenter {
    fn present(&self) -> Result<()> {
        Ok(())
    }
}
