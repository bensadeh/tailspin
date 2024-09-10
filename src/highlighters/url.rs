use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<protocol>http|https)(:)(//)(?P<host>[^:/\n\s]+)(?P<path>[/a-zA-Z0-9\-_.]*)?(?P<query>\?[^#\n ]*)?")
        .expect("Invalid regex pattern")
});

static QUERY_PARAMS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<delimiter>[?&])(?P<key>[^=]*)(?P<equal>=)(?P<value>[^&]*)").expect("Invalid regex pattern")
});

pub struct UrlHighlighter {
    http: Style,
    https: Style,
    host: Style,
    path: Style,
    query_params_key: Style,
    query_params_value: Style,
    symbols: Style,
}

impl UrlHighlighter {
    pub const fn new(
        http: Style,
        https: Style,
        host: Style,
        path: Style,
        query_params_key: Style,
        query_params_value: Style,
        symbols: Style,
    ) -> Self {
        Self {
            http,
            https,
            host,
            path,
            query_params_key,
            query_params_value,
            symbols,
        }
    }
}

impl Highlight for UrlHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.slashes < 1 || line_info.colons == 0
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        URL_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
            let protocol = &caps["protocol"];
            let protocol_style = match protocol {
                "http" => self.http,
                "https" => self.https,
                _ => Style::default(),
            };

            let query_fmt = if let Some(query) = caps.name("query") {
                QUERY_PARAMS_REGEX.replace_all(query.as_str(), |query_caps: &regex::Captures<'_>| {
                    let delimiter = &query_caps["delimiter"];
                    let key = &query_caps["key"];
                    let equal = &query_caps["equal"];
                    let value = &query_caps["value"];

                    format!(
                        "{}{}{}{}",
                        self.symbols.paint(delimiter),
                        self.query_params_key.paint(key),
                        self.symbols.paint(equal),
                        self.query_params_value.paint(value)
                    )
                })
            } else {
                Cow::Borrowed("")
            };

            format!(
                "{}://{}{}{}",
                protocol_style.paint(protocol),
                self.host.paint(&caps["host"]),
                self.path
                    .paint(caps.name("path").map(|m| m.as_str()).unwrap_or_default()),
                query_fmt
            )
        })
    }
}
