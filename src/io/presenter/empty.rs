use crate::io::controller::Presenter;
use crate::io::presenter::Present;
use miette::Result;

pub struct NoPresenter {}

impl NoPresenter {
    pub const fn get_presenter() -> Presenter {
        Presenter::None(Self {})
    }
}

impl Present for NoPresenter {
    fn present(&self) -> Result<()> {
        Ok(())
    }
}
