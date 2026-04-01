use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct KeyValueFinder {
    regex: Regex,
    key: Style,
    separator: Style,
}

impl KeyValueFinder {
    pub fn new(key: Style, separator: Style) -> Self {
        let pattern = r"(?P<space_or_start>(^)|\s)(?P<key>\w+\b)(?P<equals>=)";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded key-value regex must compile");

        Self { regex, key, separator }
    }
}

impl Finder for KeyValueFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'=', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            if let Some(k) = caps.name("key") {
                collector.push(k.start(), k.end(), self.key);
            }
            if let Some(e) = caps.name("equals") {
                collector.push(e.start(), e.end(), self.separator);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> KeyValueFinder {
        KeyValueFinder::new(Style::new().fg(Color::Red), Style::new().fg(Color::Yellow))
    }

    fn span_texts<'a>(input: &'a str, finder: &KeyValueFinder) -> Vec<&'a str> {
        let mut collector = Collector::new(0);
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
