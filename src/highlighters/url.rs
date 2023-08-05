use crate::color;
use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regexes::{QUERY_PARAMS_REGEX, URL_REGEX};
use crate::theme::Url;
use crate::types::Highlight;
use regex::Regex;

pub struct UrlHighlighter {
    http_color: String,
    https_color: String,
    host_color: String,
    path_color: String,
    query_params_key_color: String,
    query_params_value_color: String,
    symbols_color: String,
    url_regex: Regex,
    query_params_regex: Regex,
}

impl UrlHighlighter {
    pub fn new(url_group: &Url) -> Self {
        let http_color = to_ansi(&url_group.http);
        let https_color = to_ansi(&url_group.https);
        let host_color = to_ansi(&url_group.host);
        let path_color = to_ansi(&url_group.path);
        let query_params_key_color = to_ansi(&url_group.query_params_key);
        let query_params_value_color = to_ansi(&url_group.query_params_value);
        let symbols_color = to_ansi(&url_group.symbols);

        Self {
            http_color,
            https_color,
            host_color,
            path_color,
            query_params_key_color,
            query_params_value_color,
            symbols_color,
            url_regex: URL_REGEX.clone(),
            query_params_regex: QUERY_PARAMS_REGEX.clone(),
        }
    }
}

impl Highlight for UrlHighlighter {
    fn apply(&self, input: &str, line_info: &LineInfo) -> String {
        highlight_urls(
            &self.http_color,
            &self.https_color,
            &self.host_color,
            &self.path_color,
            &self.query_params_key_color,
            &self.query_params_value_color,
            &self.symbols_color,
            input,
            line_info,
            &self.url_regex,
            &self.query_params_regex,
        )
    }
}

fn highlight_urls(
    http_color: &str,
    https_color: &str,
    host_color: &str,
    path_color: &str,
    query_params_key_color: &str,
    query_params_value_color: &str,
    symbols_color: &str,
    input: &str,
    line_info: &LineInfo,
    url_regex: &Regex,
    query_params_regex: &Regex,
) -> String {
    if line_info.slashes < 1 || line_info.colons == 0 {
        return input.to_string();
    }

    let highlighted = url_regex.replace_all(input, |caps: &regex::Captures<'_>| {
        let mut output = String::new();

        if let Some(protocol) = caps.name("protocol") {
            let color = match protocol.as_str() {
                "http" => http_color,
                "https" => https_color,
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
            output.push_str(&format!("{}{}{}", host_color, host.as_str(), color::RESET));
        }

        if let Some(path) = caps.name("path") {
            output.push_str(&format!("{}{}{}", path_color, path.as_str(), color::RESET));
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
                        symbols_color,
                        delimiter,
                        query_params_key_color,
                        key,
                        symbols_color,
                        equal,
                        query_params_value_color,
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
    use colored::Color;

    #[test]
    fn test_highlight_urls() {
        let line_info = LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 2,
            double_quotes: 0,
            colons: 1,
        };

        let url_group = get_default_group();

        let highlighter = UrlHighlighter::new(&url_group);

        let input = "Visit http://www.example.com/path?param1=value1&param2=value2";
        let expected_output =
            "Visit \u{1b}[31mhttp:\u{1b}[0m//\u{1b}[0m\u{1b}[33mwww.example.com\u{1b}[0m\u{1b}[34m/path\u{1b}[0m\u{1b}[37m?\u{1b}[35mparam1\u{1b}[37m=\u{1b}[36mvalue1\u{1b}[37m&\u{1b}[35mparam2\u{1b}[37m=\u{1b}[36mvalue2\u{1b}[0m\u{1b}[0m";

        assert_eq!(highlighter.apply(input, &line_info), expected_output);
    }

    #[test]
    fn test_short_circuit_on_few_slashes() {
        let line_info = LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 1,
            double_quotes: 0,
            colons: 0,
        };

        let url_group = get_default_group();

        let highlighter = UrlHighlighter::new(&url_group);

        let input = "Visit http://www.example.com/path?param1=value1&param2=value2";
        let expected_output = "Visit http://www.example.com/path?param1=value1&param2=value2";

        assert_eq!(highlighter.apply(input, &line_info), expected_output);
    }

    #[test]
    fn test_short_circuit_on_no_colons() {
        let line_info = LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 2,
            double_quotes: 0,
            colons: 0,
        };

        let url_group = get_default_group();

        let highlighter = UrlHighlighter::new(&url_group);

        let input = "Visit http://www.example.com/path?param1=value1&param2=value2";
        let expected_output = "Visit http://www.example.com/path?param1=value1&param2=value2";

        assert_eq!(highlighter.apply(input, &line_info), expected_output);
    }

    fn get_default_group() -> Url {
        Url {
            http: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            https: Style {
                fg: Fg::Green,
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
