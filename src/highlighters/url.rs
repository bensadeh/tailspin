use crate::color;
use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regex::{QUERY_PARAMS_REGEX, URL_REGEX};
use crate::theme::Url;
use crate::types::Highlight;
use regex::Regex;

pub struct UrlHighlighter {
    url_components: UrlComponents,
    url_regex: Regex,
    query_params_regex: Regex,
}

struct UrlComponents {
    http_color: String,
    https_color: String,
    host_color: String,
    path_color: String,
    query_params_key_color: String,
    query_params_value_color: String,
    symbols_color: String,
}

impl UrlHighlighter {
    pub fn new(url_group: &Url) -> Self {
        let url_components = UrlComponents {
            http_color: to_ansi(&url_group.http),
            https_color: to_ansi(&url_group.https),
            host_color: to_ansi(&url_group.host),
            path_color: to_ansi(&url_group.path),
            query_params_key_color: to_ansi(&url_group.query_params_key),
            query_params_value_color: to_ansi(&url_group.query_params_value),
            symbols_color: to_ansi(&url_group.symbols),
        };

        Self {
            url_components,
            url_regex: URL_REGEX.clone(),
            query_params_regex: QUERY_PARAMS_REGEX.clone(),
        }
    }
}

impl Highlight for UrlHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.slashes < 1 || line_info.colons == 0 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_urls(
            &self.url_components,
            input,
            &self.url_regex,
            &self.query_params_regex,
        )
    }
}

fn highlight_urls(
    url_components: &UrlComponents,
    input: &str,
    url_regex: &Regex,
    query_params_regex: &Regex,
) -> String {
    let highlighted = url_regex.replace_all(input, |caps: &regex::Captures<'_>| {
        let mut output = String::new();

        if let Some(protocol) = caps.name("protocol") {
            let color = match protocol.as_str() {
                "http" => &url_components.http_color,
                "https" => &url_components.https_color,
                _ => color::RESET,
            };
            output.push_str(&format!(
                "{}{}:{}//{}",
                color,
                protocol.as_str(),
                color::RESET,
                color::RESET
            ));
        }

        if let Some(host) = caps.name("host") {
            output.push_str(&format!(
                "{}{}{}",
                &url_components.host_color,
                host.as_str(),
                color::RESET
            ));
        }

        if let Some(path) = caps.name("path") {
            output.push_str(&format!(
                "{}{}{}",
                &url_components.path_color,
                path.as_str(),
                color::RESET
            ));
        }

        if let Some(query) = caps.name("query") {
            let query_highlighted = query_params_regex.replace_all(
                query.as_str(),
                |query_caps: &regex::Captures<'_>| {
                    let delimiter = query_caps.name("delimiter").map_or("", |m| m.as_str());
                    let key = query_caps.name("key").map_or("", |m| m.as_str());
                    let equal = query_caps.name("equal").map_or("", |m| m.as_str());
                    let value = query_caps.name("value").map_or("", |m| m.as_str());
                    format!(
                        "{}{}{}{}{}{}{}{}",
                        &url_components.symbols_color,
                        delimiter,
                        &url_components.query_params_key_color,
                        key,
                        &url_components.symbols_color,
                        equal,
                        &url_components.query_params_value_color,
                        value
                    )
                },
            );
            output.push_str(&format!("{}{}", query_highlighted, color::RESET));
        }

        output.push_str(color::RESET);

        output
    });

    highlighted.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Fg;
    use crate::theme::{Style, Url};

    #[test]
    fn test_highlight_urls() {
        let url_group = get_default_group();

        let highlighter = UrlHighlighter::new(&url_group);

        let input = "Visit https://www.example.com/path?param1=value1&param2=value2";
        let expected_output =
            "Visit \u{1b}[31mhttps:\u{1b}[0m//\u{1b}[0m\u{1b}[33mwww.example.com\u{1b}[0m\u{1b}[34m/path\u{1b}[0m\u{1b}[37m?\u{1b}[35mparam1\u{1b}[37m=\u{1b}[36mvalue1\u{1b}[37m&\u{1b}[35mparam2\u{1b}[37m=\u{1b}[36mvalue2\u{1b}[0m\u{1b}[0m";

        assert_eq!(highlighter.apply(input), expected_output);
    }

    #[test]
    fn test_short_circuit_on_few_slashes() {
        let line_info = LineInfo {
            slashes: 1,
            ..Default::default()
        };

        let url_group = get_default_group();

        let highlighter = UrlHighlighter::new(&url_group);
        let should_short_circuit_actual = highlighter.should_short_circuit(&line_info);

        assert!(should_short_circuit_actual);
    }

    #[test]
    fn test_short_circuit_on_no_colons() {
        let line_info = LineInfo {
            slashes: 2,
            ..Default::default()
        };

        let url_group = get_default_group();
        let highlighter = UrlHighlighter::new(&url_group);

        let should_short_circuit_actual = highlighter.should_short_circuit(&line_info);

        assert!(should_short_circuit_actual);
    }

    fn get_default_group() -> Url {
        Url {
            http: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            https: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            host: Style {
                fg: Fg::Yellow,
                ..Default::default()
            },
            path: Style {
                fg: Fg::Blue,
                ..Default::default()
            },
            query_params_key: Style {
                fg: Fg::Magenta,
                ..Default::default()
            },
            query_params_value: Style {
                fg: Fg::Cyan,
                ..Default::default()
            },
            symbols: Style {
                fg: Fg::White,
                ..Default::default()
            },
        }
    }
}
