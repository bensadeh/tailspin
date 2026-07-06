use regex::{Error, Regex};

use crate::style::Style;

use super::super::span::{Collector, Finder};

/// With exactly one capture group, only the captured portion is styled
/// (falling back to the full match when the group doesn't participate);
/// otherwise the full match is styled.
#[derive(Debug, Clone)]
pub(crate) struct RegexFinder {
    regex: Regex,
    style: Style,
    single_capture_group: bool,
}

impl RegexFinder {
    pub fn new(pattern: &str, style: Style) -> Result<Self, Error> {
        let regex = Regex::new(pattern)?;
        Ok(Self {
            single_capture_group: regex.captures_len() == 2,
            regex,
            style,
        })
    }
}

impl Finder for RegexFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        for caps in self.regex.captures_iter(input) {
            let entire_match = caps.get(0).unwrap();
            let m = if self.single_capture_group {
                caps.get(1).unwrap_or(entire_match)
            } else {
                entire_match
            };
            collector.push(m.start(), m.end(), self.style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn span_texts<'a>(input: &'a str, pattern: &str) -> Vec<&'a str> {
        let finder = RegexFinder::new(pattern, Style::new().fg(Color::Red)).unwrap();
        super::super::span_texts(input, &finder)
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
