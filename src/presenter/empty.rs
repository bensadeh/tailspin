use crate::presenter::Present;

pub struct EmptyPresenter {}

impl Present for EmptyPresenter {
    fn present(&self) {
        // no-op
    }
}
