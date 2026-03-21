use crate::core::highlighter::Highlight;
use crate::core::highlighters::date_dash::DateDashHighlighter;
use crate::core::highlighters::date_time::TimeHighlighter;
use crate::core::highlighters::ip_v4::IpV4Highlighter;
use crate::core::highlighters::ip_v6::IpV6Highlighter;
use crate::core::highlighters::json::JsonHighlighter;
use crate::core::highlighters::key_value::KeyValueHighlighter;
use crate::core::highlighters::keyword::KeywordHighlighter;
use crate::core::highlighters::number::NumberHighlighter;
use crate::core::highlighters::pointer::PointerHighlighter;
use crate::core::highlighters::quote::QuoteHighlighter;
use crate::core::highlighters::regex::RegexpHighlighter;
use crate::core::highlighters::unix_path::UnixPathHighlighter;
use crate::core::highlighters::unix_process::UnixProcessHighlighter;
use crate::core::highlighters::url::UrlHighlighter;
use crate::core::highlighters::uuid::UuidHighlighter;
use ::regex::{Captures, Regex};
use nu_ansi_term::Style as NuStyle;
use std::borrow::Cow;

const RESET: &str = "\x1b[0m";

/// Pre-computed ANSI escape prefix for a style. Avoids the `Display` dispatch
/// of `style.paint(s)` on every call by writing `prefix + s + RESET` directly.
pub(crate) struct Painter {
    prefix: String,
}

impl Painter {
    pub fn new(style: NuStyle) -> Self {
        let styled = format!("{}", style.paint(""));
        let prefix = styled.replace(RESET, "");
        Self { prefix }
    }

    #[inline]
    pub fn paint(&self, buf: &mut String, s: &str) {
        if self.prefix.is_empty() {
            buf.push_str(s);
        } else {
            buf.push_str(&self.prefix);
            buf.push_str(s);
            buf.push_str(RESET);
        }
    }

    #[inline]
    pub fn paint_with_padding(&self, buf: &mut String, s: &str) {
        if self.prefix.is_empty() {
            buf.push(' ');
            buf.push_str(s);
            buf.push(' ');
        } else {
            buf.push_str(&self.prefix);
            buf.push(' ');
            buf.push_str(s);
            buf.push(' ');
            buf.push_str(RESET);
        }
    }
}

pub mod date_dash;
pub mod date_time;
pub mod ip_v4;
pub mod ip_v6;
pub mod json;
pub mod key_value;
pub mod keyword;
pub mod number;
pub mod pointer;
pub mod quote;
pub mod regex;
pub mod unix_path;
pub mod unix_process;
pub mod url;
pub mod uuid;

/// Extension trait for `Regex` that provides a zero-alloc alternative to
/// `replace_all`. Writes directly into a single buffer instead of allocating
/// a `String` per match, and returns `Cow::Borrowed` when there are no matches.
pub(crate) trait RegexExt {
    fn replace_all_cow<'a, F>(&self, input: &'a str, replacer: F) -> Cow<'a, str>
    where
        F: FnMut(&Captures<'_>, &mut String);
}

impl RegexExt for Regex {
    fn replace_all_cow<'a, F>(&self, input: &'a str, mut replacer: F) -> Cow<'a, str>
    where
        F: FnMut(&Captures<'_>, &mut String),
    {
        let mut out: Option<String> = None;
        let mut last = 0usize;

        for caps in self.captures_iter(input) {
            let m = caps.get(0).unwrap();
            let buf = out.get_or_insert_with(|| String::with_capacity(input.len() + 32));
            buf.push_str(&input[last..m.start()]);
            replacer(&caps, buf);
            last = m.end();
        }

        match out {
            Some(mut buf) => {
                buf.push_str(&input[last..]);
                Cow::Owned(buf)
            }
            None => Cow::Borrowed(input),
        }
    }
}

pub enum StaticHighlight {
    DateDash(DateDashHighlighter),
    Time(TimeHighlighter),
    IpV4(IpV4Highlighter),
    IpV6(IpV6Highlighter),
    Json(JsonHighlighter),
    KeyValue(KeyValueHighlighter),
    Keyword(KeywordHighlighter),
    Number(NumberHighlighter),
    Pointer(PointerHighlighter),
    Quote(QuoteHighlighter),
    Regexp(RegexpHighlighter),
    UnixPath(UnixPathHighlighter),
    UnixProcess(UnixProcessHighlighter),
    Url(UrlHighlighter),
    Uuid(UuidHighlighter),
}

impl StaticHighlight {
    pub fn needs_full_input(&self) -> bool {
        matches!(self, StaticHighlight::Quote(_))
    }
}

impl Highlight for StaticHighlight {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        match self {
            StaticHighlight::DateDash(h) => h.apply(input),
            StaticHighlight::Time(h) => h.apply(input),
            StaticHighlight::IpV4(h) => h.apply(input),
            StaticHighlight::IpV6(h) => h.apply(input),
            StaticHighlight::Json(h) => h.apply(input),
            StaticHighlight::KeyValue(h) => h.apply(input),
            StaticHighlight::Keyword(h) => h.apply(input),
            StaticHighlight::Number(h) => h.apply(input),
            StaticHighlight::Pointer(h) => h.apply(input),
            StaticHighlight::Quote(h) => h.apply(input),
            StaticHighlight::Regexp(h) => h.apply(input),
            StaticHighlight::UnixPath(h) => h.apply(input),
            StaticHighlight::UnixProcess(h) => h.apply(input),
            StaticHighlight::Url(h) => h.apply(input),
            StaticHighlight::Uuid(h) => h.apply(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn painter_prefix_ends_with_reset() {
        let style = NuStyle {
            foreground: Some(nu_ansi_term::Color::Red),
            ..Default::default()
        };
        let styled = format!("{}", style.paint(""));
        assert!(
            styled.ends_with(RESET),
            "nu_ansi_term output must end with RESET: {styled:?}"
        );
    }

    #[test]
    fn painter_default_style_produces_empty_prefix() {
        let painter = Painter::new(NuStyle::default());
        assert!(painter.prefix.is_empty());
    }

    #[test]
    fn painter_paint_roundtrip() {
        let painter = Painter::new(NuStyle {
            foreground: Some(nu_ansi_term::Color::Green),
            ..Default::default()
        });
        let mut buf = String::new();
        painter.paint(&mut buf, "hello");
        assert!(buf.starts_with("\x1b["));
        assert!(buf.ends_with(RESET));
        assert!(buf.contains("hello"));
    }
}
