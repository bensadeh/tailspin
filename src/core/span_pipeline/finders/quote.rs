use memchr::memchr_iter;

use crate::core::config::QuoteConfig;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct QuoteFinder {
    quote_token: u8,
    style: StyleId,
}

impl QuoteFinder {
    pub fn new(config: QuoteConfig, palette: &mut Palette) -> Self {
        Self {
            quote_token: config.quote_token,
            style: palette.intern(config.style),
        }
    }
}

impl Finder for QuoteFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        let positions: Vec<usize> = memchr_iter(self.quote_token, input.as_bytes()).collect();

        if positions.len() < 2 || !positions.len().is_multiple_of(2) {
            return;
        }

        for pair in positions.chunks(2) {
            // Span covers opening quote through closing quote (inclusive)
            collector.push(pair[0], pair[1] + 1, self.style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::span_texts;
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder(quote_token: u8) -> QuoteFinder {
        QuoteFinder::new(
            QuoteConfig {
                quote_token,
                style: Style::new().fg(Color::Yellow),
            },
            &mut Palette::new(),
        )
    }

    #[test]
    fn finds_quoted_regions() {
        let texts = span_texts(r#"hello "world" end"#, &make_finder(b'"'));
        assert_eq!(texts, [r#""world""#]);
    }

    #[test]
    fn odd_quotes_produces_no_spans() {
        assert!(span_texts(r#"hello "world end"#, &make_finder(b'"')).is_empty());
    }

    #[test]
    fn multiple_quoted_regions() {
        let texts = span_texts(r#""hello" and "world""#, &make_finder(b'"'));
        assert_eq!(texts, [r#""hello""#, r#""world""#]);
    }

    #[test]
    fn adjacent_quoted_strings() {
        // Adjacent same-style spans get coalesced by the Collector
        let texts = span_texts(r#""hello""world""#, &make_finder(b'"'));
        assert_eq!(texts, [r#""hello""world""#]);
    }

    #[test]
    fn empty_quoted_string() {
        let texts = span_texts(r#"before "" after"#, &make_finder(b'"'));
        assert_eq!(texts, [r#""""#]);
    }

    #[test]
    fn single_quote_token() {
        let texts = span_texts("msg 'hello world' done", &make_finder(b'\''));
        assert_eq!(texts, ["'hello world'"]);
    }

    #[test]
    fn three_quotes_produces_no_spans() {
        // 3 quotes is odd — should produce no spans
        assert!(span_texts(r#"a "b" c "d"#, &make_finder(b'"')).is_empty());
    }
}
