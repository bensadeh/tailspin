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

impl Highlight for StaticHighlight {
    fn apply(&self, input: &str) -> String {
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
