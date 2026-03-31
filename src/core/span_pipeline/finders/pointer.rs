use memchr::memchr2;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct PointerFinder {
    regex: Regex,
    number: Style,
    letter: Style,
    x: Style,
}

impl PointerFinder {
    pub fn new(number: Style, letter: Style, x: Style) -> Self {
        let pattern = r"(?ix)
            \b
            (?P<prefix>0x)
            (?P<first_half>[0-9a-fA-F]{8})
            \b
            |
            \b
            (?P<prefix64>0x)
            (?P<first_half64>[0-9a-fA-F]{8})
            (?P<second_half>[0-9a-fA-F]{8})
            \b
        ";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded pointer regex must compile");

        Self {
            regex,
            number,
            letter,
            x,
        }
    }

    fn push_hex_chars(&self, input: &str, offset: usize, collector: &mut Collector) {
        for (i, c) in input.char_indices() {
            let style = match c {
                '0'..='9' => self.number,
                'x' | 'X' => self.x,
                'a'..='f' | 'A'..='F' => self.letter,
                _ => continue,
            };
            collector.push(offset + i, offset + i + c.len_utf8(), style);
        }
    }
}

impl Finder for PointerFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr2(b'x', b'X', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            let prefix = caps.name("prefix").or_else(|| caps.name("prefix64")).unwrap();
            let first_half = caps.name("first_half").or_else(|| caps.name("first_half64")).unwrap();

            self.push_hex_chars(prefix.as_str(), prefix.start(), collector);
            self.push_hex_chars(first_half.as_str(), first_half.start(), collector);

            if let Some(second_half) = caps.name("second_half") {
                self.push_hex_chars(second_half.as_str(), second_half.start(), collector);
            }
        }
    }
}
