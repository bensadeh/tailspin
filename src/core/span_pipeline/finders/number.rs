use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct NumberFinder {
    regex: Regex,
    style: Style,
}

impl NumberFinder {
    pub fn new(style: Style) -> Self {
        let pattern = r"(?x)
            \b          # start of number
            \d+         # integer part
            (?:\.\d+)?  # optional fractional
            \b          # end of number
        ";

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded number regex must compile");

        Self { regex, style }
    }
}

impl Finder for NumberFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        for m in self.regex.find_iter(input) {
            collector.push(m.start(), m.end(), self.style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn finds_numbers() {
        let finder = NumberFinder::new(Style::new().fg(Color::Cyan));
        let mut collector = Collector::new(0);
        finder.find_spans("hello 42 world 3.14", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert_eq!(&"hello 42 world 3.14"[spans[0].start..spans[0].end], "42");
        assert_eq!(&"hello 42 world 3.14"[spans[1].start..spans[1].end], "3.14");
    }

    #[test]
    fn no_match_produces_no_spans() {
        let finder = NumberFinder::new(Style::new().fg(Color::Cyan));
        let mut collector = Collector::new(0);
        finder.find_spans("no numbers here", &mut collector);
        assert!(collector.into_spans().is_empty());
    }
}
