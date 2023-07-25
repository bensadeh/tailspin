use crate::color;
use crate::color::to_ansi;
use crate::config::Url;
use crate::highlighters::HighlightFn;
use crate::line_info::LineInfo;
use lazy_static::lazy_static;
use regex::Regex;

pub fn highlight(url_group: &Url) -> HighlightFn {
    let http_color = to_ansi(&url_group.http);
    let https_color = to_ansi(&url_group.https);
    let host_color = to_ansi(&url_group.host);
    let path_color = to_ansi(&url_group.path);
    let query_params_key_color = to_ansi(&url_group.query_params_key);
    let query_params_value_color = to_ansi(&url_group.query_params_value);
    let symbols_color = to_ansi(&url_group.symbols);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_urls(
            &http_color,
            &https_color,
            &host_color,
            &path_color,
            &query_params_key_color,
            &query_params_value_color,
            &symbols_color,
            input,
            line_info,
            &URL_REGEX,
            &QUERY_PARAMS_REGEX,
        )
    })
}

lazy_static! {
    static ref URL_REGEX: Regex = {
        Regex::new(
        r"(?P<protocol>http|https)(:)(//)(?P<host>[^:/\n\s]+)(?P<path>[/a-zA-Z0-9\-_.]*)?(?P<query>\?[^#\n ]*)?")
        .expect("Invalid regex pattern")
    };
    static ref QUERY_PARAMS_REGEX: Regex = {
        Regex::new(r"(?P<delimiter>[?&])(?P<key>[^=]*)(?P<equal>=)(?P<value>[^&]*)")
            .expect("Invalid query params regex pattern")
    };
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

    #[test]
    fn test_highlight_urls() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 2,
            double_quotes: 0,
            colons: 1,
        };
        let http_color = "\x1b[31m"; // Red color
        let https_color = "\x1b[32m"; // Green color
        let host_color = "\x1b[33m"; // Yellow color
        let path_color = "\x1b[34m"; // Blue color
        let query_params_key_color = "\x1b[35m"; // Magenta color
        let query_params_value_color = "\x1b[36m"; // Cyan color
        let symbols_color = "\x1b[37m"; // White color
        let input = "Visit http://www.example.com/path?param1=value1&param2=value2";

        let expected_output =
            "Visit \u{1b}[31mhttp:\u{1b}[0m//\u{1b}[0m\u{1b}[33mwww.example.com\u{1b}\
        [0m\u{1b}[34m/path\u{1b}[0m\u{1b}[37m?\u{1b}[35mparam1\u{1b}[37m=\u{1b}[36mvalue1\u{1b}\
        [37m&\u{1b}[35mparam2\u{1b}[37m=\u{1b}[36mvalue2\u{1b}[0m\u{1b}[0m";

        assert_eq!(
            highlight_urls(
                http_color,
                https_color,
                host_color,
                path_color,
                query_params_key_color,
                query_params_value_color,
                symbols_color,
                input,
                line_info,
                &URL_REGEX,
                &QUERY_PARAMS_REGEX,
            ),
            expected_output
        );
    }

    #[test]
    fn test_short_circuit_on_few_slashes() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 1,
            double_quotes: 0,
            colons: 0,
        };

        let http_color = "\x1b[31m"; // Red color
        let https_color = "\x1b[32m"; // Green color
        let host_color = "\x1b[33m"; // Yellow color
        let path_color = "\x1b[34m"; // Blue color
        let query_params_key_color = "\x1b[35m"; // Magenta color
        let query_params_value_color = "\x1b[36m"; // Cyan color
        let symbols_color = "\x1b[37m"; // White color
        let input = "Visit http://www.example.com/path?param1=value1&param2=value2";

        let expected_output = "Visit http://www.example.com/path?param1=value1&param2=value2";

        assert_eq!(
            highlight_urls(
                http_color,
                https_color,
                host_color,
                path_color,
                query_params_key_color,
                query_params_value_color,
                symbols_color,
                input,
                line_info,
                &URL_REGEX,
                &QUERY_PARAMS_REGEX,
            ),
            expected_output
        );
    }

    #[test]
    fn test_short_circuit_on_no_colons() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 2,
            double_quotes: 0,
            colons: 0,
        };

        let http_color = "\x1b[31m"; // Red color
        let https_color = "\x1b[32m"; // Green color
        let host_color = "\x1b[33m"; // Yellow color
        let path_color = "\x1b[34m"; // Blue color
        let query_params_key_color = "\x1b[35m"; // Magenta color
        let query_params_value_color = "\x1b[36m"; // Cyan color
        let symbols_color = "\x1b[37m"; // White color
        let input = "Visit http://www.example.com/path?param1=value1&param2=value2";

        let expected_output = "Visit http://www.example.com/path?param1=value1&param2=value2";

        assert_eq!(
            highlight_urls(
                http_color,
                https_color,
                host_color,
                path_color,
                query_params_key_color,
                query_params_value_color,
                symbols_color,
                input,
                line_info,
                &URL_REGEX,
                &QUERY_PARAMS_REGEX,
            ),
            expected_output
        );
    }
}
