mod core;

pub use core::{
    error::Error,
    highlighter::Highlighter,
    style::{Color, Style},
};

pub mod config {
    pub use super::core::config::*;
}
