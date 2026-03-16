use super::RegexExt;
use crate::core::config::UrlConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;

pub struct UrlHighlighter {
    url_regex: Regex,
    query_params_regex: Regex,
    http: Painter,
    https: Painter,
    host: Painter,
    path: Painter,
    query_params_key: Painter,
    query_params_value: Painter,
    symbols: Painter,
}

impl UrlHighlighter {
    pub fn new(config: UrlConfig) -> Result<Self, Error> {
        let url_pattern = r"(?x)
            (?P<protocol>https?) (:) (//)
            (?P<host>[A-Za-z0-9._\-]+)
            (?P<path>(?:/[A-Za-z0-9._~\-/%+()]*)?)           # common URL-safe chars incl parens
            (?P<query>\?[A-Za-z0-9._~\-/%+&=;,@!*()?:]*)?    # include &= and friends";
        let url_regex = RegexBuilder::new(url_pattern).unicode(false).build()?;

        let query_params_pattern = r"(?x)
            (?P<delimiter>[?&])
            (?P<key>   [A-Za-z0-9._~\-+%]*)   # allow common URL-safe chars
            (?P<equal> =)
            (?P<value> [A-Za-z0-9._~\-+%]*)";
        let query_params_regex = RegexBuilder::new(query_params_pattern).unicode(false).build()?;

        Ok(Self {
            url_regex,
            query_params_regex,
            http: Painter::new(config.http.into()),
            https: Painter::new(config.https.into()),
            host: Painter::new(config.host.into()),
            path: Painter::new(config.path.into()),
            query_params_key: Painter::new(config.query_params_key.into()),
            query_params_value: Painter::new(config.query_params_value.into()),
            symbols: Painter::new(config.symbols.into()),
        })
    }
}

/// Count how many trailing `)` characters are unbalanced within the given string.
///
/// Returns the number of excess trailing `)` that should be stripped from the URL match.
/// For example:
/// - `"/wiki/Foo_(bar)"` → 0 (balanced)
/// - `"https://example.com)"` → 1 (one unbalanced trailing paren)
/// - `"/wiki/Foo_(bar))"` → 1 (one unbalanced trailing paren)
fn count_unbalanced_trailing_parens(s: &str) -> usize {
    let open = s.chars().filter(|&c| c == '(').count();
    let close = s.chars().filter(|&c| c == ')').count();

    if close <= open {
        return 0;
    }

    let excess = close - open;
    let trailing = s.len() - s.trim_end_matches(')').len();

    trailing.min(excess)
}

impl Highlight for UrlHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.url_regex.replace_all_cow(input, |caps, buf| {
            let full_match = caps.get(0).unwrap().as_str();
            let trim_count = count_unbalanced_trailing_parens(full_match);

            if let Some(protocol) = caps.name("protocol") {
                let painter = match protocol.as_str() {
                    "http" => &self.http,
                    "https" => &self.https,
                    _ => &self.http,
                };
                painter.paint(buf, protocol.as_str());
                buf.push_str("://");
            }

            if let Some(host) = caps.name("host") {
                self.host.paint(buf, host.as_str());
            }

            if let Some(path) = caps.name("path") {
                let path_str = path.as_str();
                if caps.name("query").is_none() && trim_count > 0 {
                    let trimmed = &path_str[..path_str.len() - trim_count];
                    self.path.paint(buf, trimmed);
                } else {
                    self.path.paint(buf, path_str);
                }
            }

            if let Some(query) = caps.name("query") {
                let query_str = if trim_count > 0 {
                    &query.as_str()[..query.as_str().len() - trim_count]
                } else {
                    query.as_str()
                };
                let mut last = 0usize;
                for query_caps in self.query_params_regex.captures_iter(query_str) {
                    let m = query_caps.get(0).unwrap();
                    buf.push_str(&query_str[last..m.start()]);
                    let delimiter = query_caps.name("delimiter").map_or("", |m| m.as_str());
                    let key = query_caps.name("key").map_or("", |m| m.as_str());
                    let equal = query_caps.name("equal").map_or("", |m| m.as_str());
                    let value = query_caps.name("value").map_or("", |m| m.as_str());
                    self.symbols.paint(buf, delimiter);
                    self.query_params_key.paint(buf, key);
                    self.symbols.paint(buf, equal);
                    self.query_params_value.paint(buf, value);
                    last = m.end();
                }
                buf.push_str(&query_str[last..]);
            }

            // Append unbalanced trailing parens outside the highlighted URL
            for _ in 0..trim_count {
                buf.push(')');
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    fn make_highlighter() -> UrlHighlighter {
        UrlHighlighter::new(UrlConfig {
            http: Style::new().fg(Color::Yellow),
            https: Style::new().fg(Color::White),
            host: Style::new().fg(Color::Green),
            path: Style::new().fg(Color::Blue),
            query_params_key: Style::new().fg(Color::Magenta),
            query_params_value: Style::new().fg(Color::Cyan),
            symbols: Style::new().fg(Color::Red),
        })
        .unwrap()
    }

    #[test]
    fn test_url_highlighter() {
        let highlighter = make_highlighter();

        let cases = vec![
            (
                "https://www.openai.com/docs/api?apikey=abc123",
                "[white]https[reset]://[green]www.openai.com[reset][blue]/docs/api[reset][red]?[reset][magenta]apikey[reset][red]=[reset][cyan]abc123[reset]",
            ),
            (
                "https://api.example.org/api/v1/users?name=JohnDoe",
                "[white]https[reset]://[green]api.example.org[reset][blue]/api/v1/users[reset][red]?[reset][magenta]name[reset][red]=[reset][cyan]JohnDoe[reset]",
            ),
            (
                "http://example.com/path/to/resource?param1=value1&param2=value2",
                "[yellow]http[reset]://[green]example.com[reset][blue]/path/to/resource[reset][red]?[reset][magenta]param1[reset][red]=[reset][cyan]value1[reset][red]&[reset][magenta]param2[reset][red]=[reset][cyan]value2[reset]",
            ),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }

    #[test]
    fn test_url_with_balanced_parens_in_path() {
        let highlighter = make_highlighter();

        // Wikipedia-style URL with balanced parentheses
        let input = "https://en.wikipedia.org/wiki/Foo_(bar)";
        let expected = "[white]https[reset]://[green]en.wikipedia.org[reset][blue]/wiki/Foo_(bar)[reset]";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_wrapped_in_parens() {
        let highlighter = make_highlighter();

        // URL surrounded by parentheses — trailing ) should not be highlighted
        let input = "(https://example.com/path)";
        let expected = "([white]https[reset]://[green]example.com[reset][blue]/path[reset])";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_with_balanced_parens_wrapped_in_parens() {
        let highlighter = make_highlighter();

        // Wikipedia URL inside parentheses — inner parens balanced, outer ) excluded
        let input = "(https://en.wikipedia.org/wiki/Foo_(bar))";
        let expected = "([white]https[reset]://[green]en.wikipedia.org[reset][blue]/wiki/Foo_(bar)[reset])";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_with_query_wrapped_in_parens() {
        let highlighter = make_highlighter();

        // URL with query string inside parentheses
        let input = "(https://example.com/path?key=value)";
        let expected = "([white]https[reset]://[green]example.com[reset][blue]/path[reset][red]?[reset][magenta]key[reset][red]=[reset][cyan]value[reset])";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_not_wrapped_no_parens() {
        let highlighter = make_highlighter();

        // Plain URL without any parentheses — nothing to trim
        let input = "https://example.com/path";
        let expected = "[white]https[reset]://[green]example.com[reset][blue]/path[reset]";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_single_quote_not_included() {
        let highlighter = make_highlighter();

        // URL wrapped in single quotes — quote should not be part of URL
        let input = "'https://example.com/path'";
        let expected = "'[white]https[reset]://[green]example.com[reset][blue]/path[reset]'";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_multiple_parenthesized_urls_on_one_line() {
        let highlighter = make_highlighter();

        let input = "(https://a.com) and (https://b.com)";
        let expected = "([white]https[reset]://[green]a.com[reset][blue][reset]) and ([white]https[reset]://[green]b.com[reset][blue][reset])";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_with_parens_in_query_string() {
        let highlighter = make_highlighter();

        let input = "https://example.com/api?filter=(name)";
        let expected = "[white]https[reset]://[green]example.com[reset][blue]/api[reset][red]?[reset][magenta]filter[reset][red]=[reset][cyan][reset](name)";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_double_wrapped_in_parens() {
        let highlighter = make_highlighter();

        let input = "((https://example.com))";
        let expected = "(([white]https[reset]://[green]example.com[reset][blue][reset]))";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_url_with_nested_parens_in_path() {
        let highlighter = make_highlighter();

        let input = "https://en.wikipedia.org/wiki/Foo_(bar_(baz))";
        let expected = "[white]https[reset]://[green]en.wikipedia.org[reset][blue]/wiki/Foo_(bar_(baz))[reset]";

        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_count_unbalanced_trailing_parens() {
        assert_eq!(count_unbalanced_trailing_parens("no parens"), 0);
        assert_eq!(count_unbalanced_trailing_parens("/wiki/Foo_(bar)"), 0);
        assert_eq!(count_unbalanced_trailing_parens("https://example.com)"), 1);
        assert_eq!(count_unbalanced_trailing_parens("https://example.com))"), 2);
        assert_eq!(count_unbalanced_trailing_parens("/wiki/Foo_(bar))"), 1);
        assert_eq!(count_unbalanced_trailing_parens("/a_(b)_(c))"), 1);
    }
}
