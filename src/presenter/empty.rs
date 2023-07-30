use crate::presenter::Present;

pub struct NoPresenter {}

impl NoPresenter {
    pub fn get_presenter() -> Box<dyn Present + Send> {
        Box::new(Self {})
    }
}

impl Present for NoPresenter {
    fn present(&self) {
        // no-op
    }
}
