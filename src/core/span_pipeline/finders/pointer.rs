use super::build_regex;
use memchr::memchr2;
use regex::Regex;

use crate::core::config::PointerConfig;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct PointerFinder {
    regex: Regex,
    number: StyleId,
    letter: StyleId,
    x: StyleId,
}

impl PointerFinder {
    pub fn new(config: PointerConfig, palette: &mut Palette) -> Self {
        // Matches 0x + 8 hex (32-bit) or 0x + 16 hex (64-bit).
        // Both branches produce the same character classes, so find_iter
        // is sufficient — we just classify each byte by style.
        let pattern = r"(?i)\b0x(?:[0-9a-f]{16}|[0-9a-f]{8})\b";
        let regex = build_regex(pattern);

        Self {
            regex,
            number: palette.intern(config.number),
            letter: palette.intern(config.letter),
            x: palette.intern(config.x),
        }
    }
}

impl Finder for PointerFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr2(b'x', b'X', input.as_bytes()).is_none() {
            return;
        }

        for m in self.regex.find_iter(input) {
            let offset = m.start();
            for (i, &b) in m.as_str().as_bytes().iter().enumerate() {
                let style = match b {
                    b'0'..=b'9' => self.number,
                    b'x' | b'X' => self.x,
                    b'a'..=b'f' | b'A'..=b'F' => self.letter,
                    _ => continue,
                };
                collector.push(offset + i, offset + i + 1, style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> PointerFinder {
        PointerFinder::new(
            PointerConfig {
                number: Style::new().fg(Color::Blue),
                letter: Style::new().fg(Color::Magenta),
                x: Style::new().fg(Color::Red),
            },
            &mut Palette::new(),
        )
    }

    fn span_count(input: &str) -> usize {
        let mut collector = Collector::new();
        make_finder().find_spans(input, &mut collector);
        collector.into_spans().len()
    }

    fn matched_range(input: &str) -> Option<(usize, usize)> {
        let mut collector = Collector::new();
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
        let mut collector = Collector::new();
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
