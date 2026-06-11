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
