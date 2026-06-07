use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::core::config::UuidConfig;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct UuidFinder {
    regex: Regex,
    config: UuidConfig,
}

impl UuidFinder {
    pub fn new(config: UuidConfig) -> Self {
        let pattern = r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded UUID regex must compile");

        Self { regex, config }
    }
}

fn has_at_least_n_dashes(bytes: &[u8], n: usize) -> bool {
    let mut start = 0;
    for _ in 0..n {
        match memchr(b'-', &bytes[start..]) {
            Some(pos) => start += pos + 1,
            None => return false,
        }
    }
    true
}

impl Finder for UuidFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if !has_at_least_n_dashes(input.as_bytes(), 4) {
            return;
        }

        let UuidConfig {
            number,
            letter,
            separator,
        } = self.config;

        for m in self.regex.find_iter(input) {
            let matched = &input[m.start()..m.end()];
            for (i, c) in matched.char_indices() {
                let style = match c {
                    '0'..='9' => number,
                    'a'..='f' | 'A'..='F' => letter,
                    '-' => separator,
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
        let finder = UuidFinder::new(UuidConfig {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Yellow),
            separator: Style::new().fg(Color::Red),
        });
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
        assert_eq!(first.style, Style::new().fg(Color::Blue));
    }

    #[test]
    fn no_match_without_enough_dashes() {
        let finder = UuidFinder::new(UuidConfig {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Yellow),
            separator: Style::new().fg(Color::Red),
        });
        let mut collector = Collector::new();
        finder.find_spans("no dashes here at all", &mut collector);
        assert!(collector.into_spans().is_empty());
    }
}
