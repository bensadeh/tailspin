#![forbid(unsafe_code)]

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
//! ## Dependency usage
//!
//! When using `tailspin` as a library, disable default features to avoid pulling in
//! CLI-specific dependencies like `tokio`, `clap`, and `rayon`:
//!
//! ```toml
//! [dependencies]
//! tailspin = { version = "6.0", default-features = false }
//! ```
//!
//!
//! ## Example
//!
//! ```rust
//! use tailspin::config::*;
//! use tailspin::Highlighter;
//! use tailspin::style::{Color, Style};
//!
//! let highlighter = Highlighter::builder()
//!     .with_number_highlighter(NumberConfig {
//!         style: Style {
//!             fg: Some(Color::Cyan),
//!             ..Style::default()
//!         },
//!     })
//!     .with_quote_highlighter(QuoteConfig {
//!         quote_token: b'"',
//!         style: Style {
//!             fg: Some(Color::Yellow),
//!             ..Style::default()
//!         },
//!     })
//!     .with_uuid_highlighter(UuidConfig::default())
//!     .build()
//!     .expect("Failed to build highlighter");
//!
//! let input = "Hello 42 world";
//! let output = highlighter.apply(input);
//!
//! // "\x1b[36m" = ANSI cyan start, "\x1b[0m" = reset
//! assert_eq!(output, "Hello \x1b[36m42\x1b[0m world");
//! ```

mod core;

pub use core::highlighter::{Error, Highlighter, HighlighterBuilder};

/// Configuration support for custom highlighting themes and regex rules.
pub mod config {
    pub use super::core::config::*;
}

/// ANSI style and color definitions for highlighted output.
pub mod style {
    pub use super::core::style::{Color, Style};
}
