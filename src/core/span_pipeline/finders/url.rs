use memchr::memchr_iter;
use memchr::memmem;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct UrlFinder {
    url_regex: Regex,
    query_params_regex: Regex,
    http: Style,
    https: Style,
    host: Style,
    path: Style,
    query_params_key: Style,
    query_params_value: Style,
    symbols: Style,
}

impl UrlFinder {
    pub fn new(
        http: Style,
        https: Style,
        host: Style,
        path: Style,
        query_params_key: Style,
        query_params_value: Style,
        symbols: Style,
    ) -> Self {
        let url_pattern = r"(?x)
            (?P<protocol>https?) (:) (//)
            (?P<host>[A-Za-z0-9._\-]+)
            (?::(?P<port>\d{1,5}))?
            (?P<path>(?:/[A-Za-z0-9._~\-/%+()]*)?)
            (?P<query>\?[A-Za-z0-9._~\-/%+&=;,@!*()?:]*)?";
        let url_regex = RegexBuilder::new(url_pattern)
            .unicode(false)
            .build()
            .expect("hardcoded URL regex must compile");

        let query_params_pattern = r"(?x)
            (?P<delimiter>[?&])
            (?P<key>   [A-Za-z0-9._~\-+%]*)
            (?P<equal> =)
            (?P<value> [A-Za-z0-9._~\-+%]*)";
        let query_params_regex = RegexBuilder::new(query_params_pattern)
            .unicode(false)
            .build()
            .expect("hardcoded URL query-params regex must compile");

        Self {
            url_regex,
            query_params_regex,
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

fn count_unbalanced_trailing_parens(s: &str) -> usize {
    let open = memchr_iter(b'(', s.as_bytes()).count();
    let close = memchr_iter(b')', s.as_bytes()).count();

    if close <= open {
        return 0;
    }

    let excess = close - open;
    let trailing = s.len() - s.trim_end_matches(')').len();

    trailing.min(excess)
}

impl Finder for UrlFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memmem::find(input.as_bytes(), b"://").is_none() {
            return;
        }

        for caps in self.url_regex.captures_iter(input) {
            let full_match = caps.get(0).unwrap();
            let full_str = full_match.as_str();
            let trim_count = count_unbalanced_trailing_parens(full_str);

            if let Some(protocol) = caps.name("protocol") {
                let style = if protocol.as_str() == "https" {
                    self.https
                } else {
                    self.http
                };
                collector.push(protocol.start(), protocol.end(), style);
                // "://" is not styled — left as plain text
            }

            if let Some(host) = caps.name("host") {
                collector.push(host.start(), host.end(), self.host);
            }

            if let Some(port) = caps.name("port") {
                // Style the colon before the port as a symbol
                collector.push(port.start() - 1, port.start(), self.symbols);
                collector.push(port.start(), port.end(), self.host);
            }

            if let Some(path) = caps.name("path") {
                let end = if caps.name("query").is_none() && trim_count > 0 {
                    path.end() - trim_count
                } else {
                    path.end()
                };
                if path.start() < end {
                    collector.push(path.start(), end, self.path);
                }
            }

            if let Some(query) = caps.name("query") {
                let query_end = if trim_count > 0 {
                    query.end() - trim_count
                } else {
                    query.end()
                };
                let query_str = &input[query.start()..query_end];
                let query_offset = query.start();

                for query_caps in self.query_params_regex.captures_iter(query_str) {
                    if let Some(d) = query_caps.name("delimiter") {
                        collector.push(query_offset + d.start(), query_offset + d.end(), self.symbols);
                    }
                    if let Some(k) = query_caps.name("key")
                        && !k.as_str().is_empty()
                    {
                        collector.push(query_offset + k.start(), query_offset + k.end(), self.query_params_key);
                    }
                    if let Some(e) = query_caps.name("equal") {
                        collector.push(query_offset + e.start(), query_offset + e.end(), self.symbols);
                    }
                    if let Some(v) = query_caps.name("value")
                        && !v.as_str().is_empty()
                    {
                        collector.push(
                            query_offset + v.start(),
                            query_offset + v.end(),
                            self.query_params_value,
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn finder() -> UrlFinder {
        UrlFinder::new(
            Style::new().fg(Color::Red),
            Style::new().fg(Color::Green),
            Style::new().fg(Color::Blue),
            Style::new().fg(Color::Cyan),
            Style::new().fg(Color::Magenta),
            Style::new().fg(Color::Yellow),
            Style::new().fg(Color::White),
        )
    }

    fn spans_text<'a>(input: &'a str, finder: &UrlFinder) -> Vec<&'a str> {
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn matches_url_with_port() {
        let f = finder();
        let input = "http://localhost:8080/path";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"http"));
        assert!(texts.contains(&"localhost"));
        assert!(texts.contains(&":"));
        assert!(texts.contains(&"8080"));
        assert!(texts.contains(&"/path"));
    }

    #[test]
    fn matches_url_without_port() {
        let f = finder();
        let input = "https://example.com/foo";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"https"));
        assert!(texts.contains(&"example.com"));
        assert!(texts.contains(&"/foo"));
        assert!(!texts.contains(&":")); // no port colon
    }

    #[test]
    fn matches_url_with_port_and_query() {
        let f = finder();
        let input = "http://api.dev:3000/v1?key=val";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"api.dev"));
        assert!(texts.contains(&"3000"));
        assert!(texts.contains(&"key"));
        assert!(texts.contains(&"val"));
    }

    #[test]
    fn no_match_returns_no_spans() {
        let f = finder();
        let mut collector = Collector::new(0);
        f.find_spans("no urls here", &mut collector);
        assert!(collector.into_spans().is_empty());
    }

    #[test]
    fn url_wrapped_in_parens() {
        // Trailing ) should be excluded when it's unbalanced
        let f = finder();
        let input = "(http://example.com/path)";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"http"));
        assert!(texts.contains(&"example.com"));
        assert!(texts.contains(&"/path"));
        // The closing paren should NOT be part of any span
        assert!(!texts.contains(&")"));
    }

    #[test]
    fn url_with_balanced_parens_in_path() {
        // Wikipedia-style URLs with balanced parens should keep them
        let f = finder();
        let input = "http://en.wikipedia.org/wiki/Rust_(programming_language)";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"http"));
        assert!(texts.contains(&"en.wikipedia.org"));
        // Path should include the balanced parens
        let path_span = texts.iter().find(|t| t.contains("Rust_(")).unwrap();
        assert!(path_span.contains("language)"));
    }

    #[test]
    fn url_with_query_wrapped_in_parens() {
        let f = finder();
        let input = "(http://example.com/path?key=val)";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"key"));
        assert!(texts.contains(&"val"));
        assert!(!texts.contains(&")"));
    }

    #[test]
    fn multiple_query_params() {
        let f = finder();
        let input = "http://api.dev/v1?a=1&b=2";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"a"));
        assert!(texts.contains(&"1"));
        assert!(texts.contains(&"b"));
        assert!(texts.contains(&"2"));
    }

    #[test]
    fn url_with_balanced_parens_wrapped_in_parens() {
        let f = finder();
        let input = "(https://en.wikipedia.org/wiki/Foo_(bar))";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"https"));
        assert!(texts.contains(&"en.wikipedia.org"));
        // The outer closing paren should be excluded
        let path_span = texts.iter().find(|t| t.contains("Foo_(")).unwrap();
        assert!(path_span.ends_with("bar)"));
    }

    #[test]
    fn url_in_single_quotes() {
        let f = finder();
        let input = "'https://example.com/path'";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"https"));
        assert!(texts.contains(&"example.com"));
        // Single quote should not be part of any span
        assert!(!texts.iter().any(|t| t.contains('\'')));
    }

    #[test]
    fn multiple_parenthesized_urls() {
        let f = finder();
        let input = "(https://a.com/x) and (https://b.com/y)";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"a.com"));
        assert!(texts.contains(&"b.com"));
    }

    #[test]
    fn parens_in_query_string() {
        let f = finder();
        let input = "https://example.com/api?filter=(name)";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"https"));
        assert!(texts.contains(&"example.com"));
    }

    #[test]
    fn double_paren_wrapping() {
        let f = finder();
        let input = "((https://example.com))";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"https"));
        assert!(texts.contains(&"example.com"));
        // Neither closing paren should be in any span
        assert!(!texts.iter().any(|t| t.contains(')')));
    }

    #[test]
    fn nested_parens_in_path() {
        let f = finder();
        let input = "https://en.wikipedia.org/wiki/Foo_(bar_(baz))";
        let texts = spans_text(input, &f);
        assert!(texts.contains(&"https"));
        let path_span = texts.iter().find(|t| t.contains("Foo_(")).unwrap();
        assert!(path_span.contains("baz))"));
    }

    #[test]
    fn count_parens_balanced() {
        assert_eq!(count_unbalanced_trailing_parens("foo(bar)"), 0);
    }

    #[test]
    fn count_parens_one_trailing() {
        assert_eq!(count_unbalanced_trailing_parens("foo)"), 1);
    }

    #[test]
    fn count_parens_nested_balanced() {
        assert_eq!(count_unbalanced_trailing_parens("a(b(c))"), 0);
    }

    #[test]
    fn count_parens_double_trailing() {
        assert_eq!(count_unbalanced_trailing_parens("a(b))"), 1);
    }

    #[test]
    fn count_parens_two_unbalanced() {
        assert_eq!(count_unbalanced_trailing_parens("https://example.com))"), 2);
    }

    #[test]
    fn count_parens_mixed_balanced_and_trailing() {
        assert_eq!(count_unbalanced_trailing_parens("/a_(b)_(c))"), 1);
    }
}
