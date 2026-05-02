use regex::{Error, Regex};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct RegexFinder {
    regex: Regex,
    style: Style,
}

impl RegexFinder {
    pub fn new(pattern: &str, style: Style) -> Result<Self, Error> {
        Ok(Self {
            regex: Regex::new(pattern)?,
            style,
        })
    }
}

impl Finder for RegexFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        let capture_groups = self.regex.captures_len() - 1;

        for caps in self.regex.captures_iter(input) {
            if let Some(entire_match) = caps.get(0) {
                match capture_groups {
                    1 => {
                        if let Some(captured) = caps.get(1) {
                            collector.push(captured.start(), captured.end(), self.style);
                        } else {
                            collector.push(entire_match.start(), entire_match.end(), self.style);
                        }
                    }
                    _ => {
                        collector.push(entire_match.start(), entire_match.end(), self.style);
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

    fn span_texts<'a>(input: &'a str, pattern: &str) -> Vec<&'a str> {
        let finder = RegexFinder::new(pattern, Style::new().fg(Color::Red)).unwrap();
        let mut collector = Collector::new();
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn simple_pattern() {
        let texts = span_texts("hello world", "world");
        assert_eq!(texts, ["world"]);
    }

    #[test]
    fn capture_group_styles_only_group() {
        // With one capture group, only the captured portion is styled
        let texts = span_texts("got a errorwarning here", "(error)?warning");
        assert_eq!(texts, ["error"]);
    }

    #[test]
    fn optional_group_not_participating_styles_full_match() {
        // When the optional group doesn't participate, the full match is styled
        let texts = span_texts("got a warning here", "(error)?warning");
        assert_eq!(texts, ["warning"]);
    }

    #[test]
    fn no_match_no_spans() {
        let texts = span_texts("nothing here", "xyz");
        assert!(texts.is_empty());
    }

    #[test]
    fn multiple_matches() {
        let texts = span_texts("abc 123 def 456", r"\d+");
        assert_eq!(texts, ["123", "456"]);
    }
}
