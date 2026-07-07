//! One finder per pattern kind, all built to the same shape: the constructor
//! takes the finder's config plus the pipeline's `Palette`, interns each
//! configured style once, and keeps the returned `StyleId` handles — style
//! resolution happens here so the per-line hot path never looks anything up.
//! `find_spans` scans the original unstyled input and pushes
//! `(start, end, style)` byte ranges into the `Collector`; finders never see
//! each other's spans. `number.rs` is the minimal example.

use ::regex::{Regex, RegexBuilder};

pub(crate) mod date_dash;
pub(crate) mod date_time;
pub(crate) mod email;
pub(crate) mod ip_v4;
pub(crate) mod ip_v6;
pub(crate) mod json;
pub(crate) mod jvm_stack;
pub(crate) mod key_value;
pub(crate) mod keyword;
pub(crate) mod number;
pub(crate) mod pointer;
pub(crate) mod quote;
pub(crate) mod regex;
pub(crate) mod unix_path;
pub(crate) mod unix_process;
pub(crate) mod url;
pub(crate) mod uuid;

/// Hardcoded finder regexes are byte-mode: `\w`/`\d`/`\s`/`\b` stay ASCII and
/// skip the Unicode tables in the hot path.
pub(crate) fn build_regex(pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .unicode(false)
        .build()
        .expect("hardcoded finder regex must compile")
}

/// The texts of all spans a finder produces for `input`.
#[cfg(test)]
pub(crate) fn span_texts<'a>(input: &'a str, finder: &impl super::span::Finder) -> Vec<&'a str> {
    let mut collector = super::span::Collector::new();
    finder.find_spans(input, &mut collector);
    collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
}
