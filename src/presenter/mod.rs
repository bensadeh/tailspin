pub mod empty;
pub mod less;

pub struct Presenter {
    presenter: Box<dyn Present>,
}

pub trait Present: Send {
    fn present(&self);
}
