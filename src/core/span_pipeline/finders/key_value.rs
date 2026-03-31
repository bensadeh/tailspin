use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct KeyValueFinder {
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
