use super::build_regex;
use memchr::memchr_iter;
use regex::Regex;

use crate::core::config::UuidConfig;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct UuidFinder {
    regex: Regex,
    number: StyleId,
    letter: StyleId,
    separator: StyleId,
}

impl UuidFinder {
    pub fn new(config: UuidConfig, palette: &mut Palette) -> Self {
        let pattern = r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b";
        let regex = build_regex(pattern);

        Self {
            regex,
            number: palette.intern(config.number),
            letter: palette.intern(config.letter),
            separator: palette.intern(config.separator),
        }
    }
}

impl Finder for UuidFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr_iter(b'-', input.as_bytes()).nth(3).is_none() {
            return;
        }

        for m in self.regex.find_iter(input) {
            let matched = m.as_str();
            for (i, c) in matched.char_indices() {
                let style = match c {
                    '0'..='9' => self.number,
                    'a'..='f' | 'A'..='F' => self.letter,
                    '-' => self.separator,
                    _ => continue,
                };
                collector.push(m.start() + i, m.start() + i + c.len_utf8(), style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Style};

    #[test]
    fn finds_uuid_with_coalesced_spans() {
        let mut palette = Palette::new();
        let finder = UuidFinder::new(
            UuidConfig {
                number: Style::new().fg(Color::Blue),
                letter: Style::new().fg(Color::Yellow),
                separator: Style::new().fg(Color::Red),
            },
            &mut palette,
        );
        let input = "id=550e8400-e29b-41d4-a716-446655440000 done";
        let mut collector = Collector::new();
        finder.find_spans(input, &mut collector);

        let spans = collector.into_spans();

        // Should be coalesced: "550" (digits), "e" (letter), "8400" (digits), "-" (dash), ...
        // Much fewer than 36 individual spans
        assert!(spans.len() < 36);
        assert!(spans.len() > 5);

        // Verify first span covers leading digits "550"
        let first = &spans[0];
        assert_eq!(&input[first.start..first.end], "550");
        assert_eq!(first.style, palette.intern(Style::new().fg(Color::Blue)));
    }

    #[test]
    fn no_match_without_enough_dashes() {
        let finder = UuidFinder::new(
            UuidConfig {
                number: Style::new().fg(Color::Blue),
                letter: Style::new().fg(Color::Yellow),
                separator: Style::new().fg(Color::Red),
            },
            &mut Palette::new(),
        );
        assert!(super::super::span_texts("no dashes here at all", &finder).is_empty());
    }
}
