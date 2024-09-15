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

    fn apply(&self, input: &str) -> String {
        let highlighted = URL_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
            let mut output = String::new();

            if let Some(protocol) = caps.name("protocol") {
                let style = match protocol.as_str() {
                    "http" => self.http,
                    "https" => self.https,
                    _ => Style::default(),
                };
                output.push_str(&format!("{}://", style.paint(protocol.as_str())));
            }

            if let Some(host) = caps.name("host") {
                output.push_str(&format!("{}", self.host.paint(host.as_str())));
            }

            if let Some(path) = caps.name("path") {
                output.push_str(&format!("{}", self.path.paint(path.as_str())));
            }

            if let Some(query) = caps.name("query") {
                let query_highlighted =
                    QUERY_PARAMS_REGEX.replace_all(query.as_str(), |query_caps: &regex::Captures<'_>| {
                        let delimiter = query_caps.name("delimiter").map_or("", |m| m.as_str());
                        let key = query_caps.name("key").map_or("", |m| m.as_str());
                        let equal = query_caps.name("equal").map_or("", |m| m.as_str());
                        let value = query_caps.name("value").map_or("", |m| m.as_str());
                        format!(
                            "{}{}{}{}",
                            self.symbols.paint(delimiter),
                            self.query_params_key.paint(key),
                            self.symbols.paint(equal),
                            self.query_params_value.paint(value)
                        )
                    });
                output.push_str(&format!("{}", query_highlighted));
            }

            output
        });

        highlighted.into_owned()
    }
}
