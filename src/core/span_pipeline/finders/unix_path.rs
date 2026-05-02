use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct UnixPathFinder {
    regex: Regex,
    segment: Style,
    separator: Style,
}

impl UnixPathFinder {
    pub fn new(segment: Style, separator: Style) -> Self {
        // The (?:^|\s) anchor is zero-width at start-of-string or consumes one
        // whitespace byte. We use find_iter and skip that leading byte manually.
        let pattern = r"(?x)
            (?:^|\s)
            (?:\./|~/|//|/)
            [\w.-]+
            (?:/[\w.-]+)+
        ";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded Unix path regex must compile");

        Self {
            regex,
            segment,
            separator,
        }
    }
}

impl Finder for UnixPathFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'/', input.as_bytes()).is_none() {
            return;
        }

        for m in self.regex.find_iter(input) {
            let bytes = m.as_str().as_bytes();

            // Skip the leading whitespace consumed by (?:^|\s)
            let skip = usize::from(!matches!(bytes[0], b'.' | b'~' | b'/'));
            let offset = m.start() + skip;
            let path = &bytes[skip..];

            let mut seg_start = None;

            for (i, &b) in path.iter().enumerate() {
                if b == b'/' {
                    if let Some(start) = seg_start.take() {
                        collector.push(offset + start, offset + i, self.segment);
                    }
                    collector.push(offset + i, offset + i + 1, self.separator);
                } else if seg_start.is_none() {
                    seg_start = Some(i);
                }
            }

            if let Some(start) = seg_start {
                collector.push(offset + start, offset + path.len(), self.segment);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> UnixPathFinder {
        UnixPathFinder::new(Style::new().fg(Color::Green), Style::new().fg(Color::Yellow))
    }

    fn span_texts<'a>(input: &'a str, finder: &UnixPathFinder) -> Vec<&'a str> {
        let mut collector = Collector::new();
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn absolute_path() {
        let texts = span_texts("/user/local", &make_finder());
        assert_eq!(texts, ["/", "user", "/", "local"]);
    }

    #[test]
    fn deep_path() {
        let texts = span_texts("/var/log/nginx/error.log", &make_finder());
        assert_eq!(texts, ["/", "var", "/", "log", "/", "nginx", "/", "error.log"]);
    }

    #[test]
    fn home_relative_path() {
        let texts = span_texts("~/projects/rust/tailspin", &make_finder());
        assert_eq!(texts, ["~", "/", "projects", "/", "rust", "/", "tailspin"]);
    }

    #[test]
    fn dot_relative_path() {
        let texts = span_texts("./a/b", &make_finder());
        assert_eq!(texts, [".", "/", "a", "/", "b"]);
    }

    #[test]
    fn network_path() {
        let texts = span_texts("//network/share", &make_finder());
        // Adjacent separator slashes coalesce into one span
        assert_eq!(texts, ["//", "network", "/", "share"]);
    }

    #[test]
    fn hidden_directory() {
        let texts = span_texts("/path/.hidden/file", &make_finder());
        assert_eq!(texts, ["/", "path", "/", ".hidden", "/", "file"]);
    }

    #[test]
    fn path_in_context() {
        let texts = span_texts("See /etc/hosts please", &make_finder());
        assert_eq!(texts, ["/", "etc", "/", "hosts"]);
    }

    #[test]
    fn trailing_slash_not_highlighted() {
        let texts = span_texts("/usr/local/", &make_finder());
        // The path segments and separators should be found, but trailing "/" is not part of the match
        assert!(texts.contains(&"usr"));
        assert!(texts.contains(&"local"));
        // The matched text should not end with a trailing slash
        let finder = make_finder();
        let mut collector = Collector::new();
        finder.find_spans("/usr/local/", &mut collector);
        let spans = collector.into_spans();
        let last = spans.last().unwrap();
        assert_eq!(&"/usr/local/"[last.start..last.end], "local");
    }

    #[test]
    fn three_segments_without_leading_slash_no_match() {
        assert!(span_texts("a/b/c", &make_finder()).is_empty());
    }

    #[test]
    fn single_segment_no_match() {
        assert!(span_texts("justtext", &make_finder()).is_empty());
    }

    #[test]
    fn two_segments_without_leading_slash_no_match() {
        assert!(span_texts("name/name", &make_finder()).is_empty());
    }

    #[test]
    fn slash_separated_numbers_no_match() {
        assert!(span_texts("123/234/345/456", &make_finder()).is_empty());
    }
}
