pub mod highlighter;

pub use highlighter::{
    config::*,
    core::Highlighter,
    error::Error,
    style::{Color, Style},
};
