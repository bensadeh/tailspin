use memchr::memchr_iter;

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct QuoteFinder {
    quote_token: u8,
    style: Style,
}

impl QuoteFinder {
    pub fn new(quote_token: u8, style: Style) -> Self {
        Self { quote_token, style }
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
    use super::*;
    use crate::style::Color;

    #[test]
    fn finds_quoted_regions() {
        let finder = QuoteFinder::new(b'"', Style::new().fg(Color::Yellow));
        let input = r#"hello "world" end"#;
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert_eq!(&input[spans[0].start..spans[0].end], r#""world""#);
    }

    #[test]
    fn odd_quotes_produces_no_spans() {
        let finder = QuoteFinder::new(b'"', Style::new().fg(Color::Yellow));
        let input = r#"hello "world end"#;
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);
        assert!(collector.into_spans().is_empty());
    }

    #[test]
    fn multiple_quoted_regions() {
        let finder = QuoteFinder::new(b'"', Style::new().fg(Color::Yellow));
        let input = r#""hello" and "world""#;
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert_eq!(&input[spans[0].start..spans[0].end], r#""hello""#);
        assert_eq!(&input[spans[1].start..spans[1].end], r#""world""#);
    }
}
