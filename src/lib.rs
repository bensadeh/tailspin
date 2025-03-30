mod highlighter;

pub use highlighter::{
    core::Highlighter,
    error::Error,
    style::{Color, Style},
};

pub mod config {
    pub use super::highlighter::config::*;
}
