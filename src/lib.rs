mod core;

pub use core::{
    core::Highlighter,
    error::Error,
    style::{Color, Style},
};

pub mod config {
    pub use super::core::config::*;
}
