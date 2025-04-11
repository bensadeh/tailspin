//! <p align="center">
//!   <a href="https://github.com/bensadeh/tailspin">
//!     <img src="https://raw.githubusercontent.com/bensadeh/tailspin/main/assets/tailspin.png" alt="tailspin logo" width="250" />
//!   </a>
//! </p>
//!
//! #
//!
//! `tailspin` is a log file highlighter. This crate exposes the [`Highlighter`] type,
//! allowing you to programmatically apply the same pattern-driven highlighting used by the CLI.
//!
//! In order to configure the highlighter, use the [`HighlighterBuilder`]. Otherwise, use
//! [`Highlighter::default()`](crate::Highlighter::default) for reasonable defaults.
//!
//!
//! ## Example
//!
//! ```rust
//! use tailspin::config::*;
//! use tailspin::Highlighter;
//! use tailspin::style::{Color, Style};
//!
//! let mut builder = Highlighter::builder();
//!
//! builder
//!     .with_number_highlighter(NumberConfig {
//!         style: Style {
//!             fg: Some(Color::Cyan),
//!             ..Style::default()
//!         },
//!     })
//!     .with_quote_highlighter(QuotesConfig {
//!         quotes_token: '"',
//!         style: Style {
//!             fg: Some(Color::Yellow),
//!             ..Style::default()
//!         },
//!     })
//!     .with_uuid_highlighter(UuidConfig::default());
//!    
//! // Using the highlight builder can fail if the regexes inside don't compile
//! let highlighter = match builder.build() {
//!     Ok(h) => h,
//!     Err(_) => panic!("Failed to build highlighter"),
//! };
//!
//! let input = "Hello 42 world";
//! let output = highlighter.apply(input);
//!
//! // "\x1b[36m" = ANSI cyan start, "\x1b[0m" = reset
//! assert_eq!(output, "Hello \x1b[36m42\x1b[0m world");
//! ```

mod core;

pub use core::{
    error::Error,
    highlighter::{Highlighter, HighlighterBuilder},
};

/// Configuration support for custom highlighting themes and regex rules.
pub mod config {
    pub use super::core::config::*;
}

/// ANSI style and color definitions for highlighted output.
pub mod style {
    pub use super::core::style::{Color, Style};
}
