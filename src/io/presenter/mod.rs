pub mod pager;

use crate::io::presenter::pager::Pager;

pub enum Presenter {
    Pager(Pager),
    Stdout,
}
