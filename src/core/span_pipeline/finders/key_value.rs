use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::core::config::KeyValueConfig;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct KeyValueFinder {
    regex: Regex,
    config: KeyValueConfig,
}

impl KeyValueFinder {
    pub fn new(config: KeyValueConfig) -> Self {
        // The (?:^|\s) anchor is zero-width at start-of-string or consumes one
        // whitespace byte. We use find_iter and skip that leading byte manually.
        let pattern = r"(?:^|\s)\w+\b=";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded key-value regex must compile");

        Self { regex, config }
    }
}

impl Finder for KeyValueFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'=', input.as_bytes()).is_none() {
            return;
        }

        let KeyValueConfig { key, separator } = self.config;

        for m in self.regex.find_iter(input) {
            let bytes = m.as_str().as_bytes();
            let skip = usize::from(bytes[0].is_ascii_whitespace());
            let s = m.start() + skip;

            // Match structure (after skip): key=
            collector.push(s, m.end() - 1, key);
            collector.push(m.end() - 1, m.end(), separator);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> KeyValueFinder {
        KeyValueFinder::new(KeyValueConfig {
            key: Style::new().fg(Color::Red),
            separator: Style::new().fg(Color::Yellow),
        })
    }

    fn span_texts<'a>(input: &'a str, finder: &KeyValueFinder) -> Vec<&'a str> {
        let mut collector = Collector::new();
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn basic_key_value() {
        let texts = span_texts("Entry key=value", &make_finder());
        assert_eq!(texts, ["key", "="]);
    }

    #[test]
    fn multiple_key_values() {
        let texts = span_texts("host=localhost port=8080", &make_finder());
        assert_eq!(texts, ["host", "=", "port", "="]);
    }

    #[test]
    fn key_value_at_start_of_line() {
        let texts = span_texts("key=value", &make_finder());
        assert_eq!(texts, ["key", "="]);
    }

    #[test]
    fn no_key_value_no_match() {
        assert!(span_texts("No numbers here!", &make_finder()).is_empty());
    }
}
