use memchr::memchr2;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct PointerFinder {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> PointerFinder {
        PointerFinder::new(
            Style::new().fg(Color::Blue),
            Style::new().fg(Color::Magenta),
            Style::new().fg(Color::Red),
        )
    }

    fn span_count(input: &str) -> usize {
        let mut collector = Collector::new(0);
        make_finder().find_spans(input, &mut collector);
        collector.into_spans().len()
    }

    fn matched_range(input: &str) -> Option<(usize, usize)> {
        let mut collector = Collector::new(0);
        make_finder().find_spans(input, &mut collector);
        let spans = collector.into_spans();
        if spans.is_empty() {
            None
        } else {
            Some((spans.first().unwrap().start, spans.last().unwrap().end))
        }
    }

    #[test]
    fn pointer_32bit() {
        let (start, end) = matched_range("0x8c2a0aeb").unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 10); // "0x" + 8 hex chars
    }

    #[test]
    fn pointer_64bit() {
        let (start, end) = matched_range("0xd7b3b2f446e2c21b").unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 18); // "0x" + 16 hex chars
    }

    #[test]
    fn pointer_distinguishes_digits_and_letters() {
        let finder = make_finder();
        // Must be exactly 8 hex chars after 0x to match 32-bit pattern
        let input = "0xab12cd34";
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);
        let spans = collector.into_spans();
        // Per-char spans coalesced by style: "0" (number), "x" (x), "ab" (letter),
        // "12" (number), "cd" (letter), "34" (number)
        assert!(spans.len() >= 4);
    }

    #[test]
    fn no_pointer_no_match() {
        assert_eq!(span_count("No numbers here!"), 0);
    }
}
