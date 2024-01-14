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
    pub fn new(
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

            // output.push_str(color::RESET);

            output
        });

        highlighted.into_owned()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::color::Fg;
//
//     #[test]
//     fn test_highlight_urls() {
//         let url_group = get_default_group();
//
//         let highlighter = UrlHighlighter::new(&url_group);
//
//         let input = "Visit https://www.example.com/path?param1=value1&param2=value2";
//         let expected_output =
//             "Visit \u{1b}[31mhttps:\u{1b}[0m//\u{1b}[0m\u{1b}[33mwww.example.com\u{1b}[0m\u{1b}[34m/path\u{1b}[0m\u{1b}[37m?\u{1b}[35mparam1\u{1b}[37m=\u{1b}[36mvalue1\u{1b}[37m&\u{1b}[35mparam2\u{1b}[37m=\u{1b}[36mvalue2\u{1b}[0m\u{1b}[0m";
//
//         assert_eq!(highlighter.apply(input), expected_output);
//     }
//
//     #[test]
//     fn test_short_circuit_on_few_slashes() {
//         let line_info = LineInfo {
//             slashes: 1,
//             ..Default::default()
//         };
//
//         let url_group = get_default_group();
//
//         let highlighter = UrlHighlighter::new(&url_group);
//         let should_short_circuit_actual = highlighter.should_short_circuit(&line_info);
//
//         assert!(should_short_circuit_actual);
//     }
//
//     #[test]
//     fn test_short_circuit_on_no_colons() {
//         let line_info = LineInfo {
//             slashes: 2,
//             ..Default::default()
//         };
//
//         let url_group = get_default_group();
//         let highlighter = UrlHighlighter::new(&url_group);
//
//         let should_short_circuit_actual = highlighter.should_short_circuit(&line_info);
//
//         assert!(should_short_circuit_actual);
//     }
//
//     fn get_default_group() -> Url {
//         Url {
//             http: Style {
//                 fg: Fg::Red,
//                 ..Default::default()
//             },
//             https: Style {
//                 fg: Fg::Red,
//                 ..Default::default()
//             },
//             host: Style {
//                 fg: Fg::Yellow,
//                 ..Default::default()
//             },
//             path: Style {
//                 fg: Fg::Blue,
//                 ..Default::default()
//             },
//             query_params_key: Style {
//                 fg: Fg::Magenta,
//                 ..Default::default()
//             },
//             query_params_value: Style {
//                 fg: Fg::Cyan,
//                 ..Default::default()
//             },
//             symbols: Style {
//                 fg: Fg::White,
//                 ..Default::default()
//             },
//             disabled: false,
//         }
//     }
// }
