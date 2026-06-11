use super::build_regex;
use memchr::{memchr, memchr3};
use regex::Regex;

use crate::core::config::NumberConfig;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct NumberFinder {
    regex: Regex,
    config: NumberConfig,
}

impl NumberFinder {
    pub fn new(config: NumberConfig) -> Self {
        let pattern = r"(?x)
            \b          # start of number
            \d+         # integer part
            (?:\.\d+)?  # optional fractional
            \b          # end of number
        ";

        let regex = build_regex(pattern);

        Self { regex, config }
    }
}

impl Finder for NumberFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        let bytes = input.as_bytes();
        if memchr3(b'0', b'1', b'2', bytes).is_none()
            && memchr3(b'3', b'4', b'5', bytes).is_none()
            && memchr3(b'6', b'7', b'8', bytes).is_none()
            && memchr(b'9', bytes).is_none()
        {
            return;
        }

        let NumberConfig { style } = self.config;

        for m in self.regex.find_iter(input) {
            collector.push(m.start(), m.end(), style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Style};

    #[test]
    fn finds_numbers() {
        let finder = NumberFinder::new(NumberConfig {
            style: Style::new().fg(Color::Cyan),
        });
        let mut collector = Collector::new();
        finder.find_spans("hello 42 world 3.14", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert_eq!(&"hello 42 world 3.14"[spans[0].start..spans[0].end], "42");
        assert_eq!(&"hello 42 world 3.14"[spans[1].start..spans[1].end], "3.14");
    }

    #[test]
    fn no_match_produces_no_spans() {
        let finder = NumberFinder::new(NumberConfig {
            style: Style::new().fg(Color::Cyan),
        });
        let mut collector = Collector::new();
        finder.find_spans("no numbers here", &mut collector);
        assert!(collector.into_spans().is_empty());
    }
}
