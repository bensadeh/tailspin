use aho_corasick::{AhoCorasick, AhoCorasickBuilder, BuildError, MatchKind};

use crate::core::config::KeywordConfig;
use crate::style::Style;

use super::super::span::{Collector, Finder};

/// Matches all configured keywords with a single automaton; each pattern
/// carries the style of the config it came from.
#[derive(Debug)]
pub(crate) struct KeywordFinder {
    ac: AhoCorasick,
    styles: Vec<Style>,
}

impl KeywordFinder {
    pub fn new(configs: &[KeywordConfig]) -> Result<Self, BuildError> {
        let words = configs.iter().flat_map(|config| &config.words);
        let styles = configs
            .iter()
            .flat_map(|config| config.words.iter().map(|_| config.style))
            .collect();

        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(words)?;

        Ok(Self { ac, styles })
    }
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_uppercase() || b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'
}

fn is_word_boundary(hay: &[u8], start: usize, end: usize) -> bool {
    let left_ok = start == 0 || !is_word_byte(hay[start - 1]);
    let right_ok = end == hay.len() || !is_word_byte(hay[end]);
    left_ok && right_ok
}

impl Finder for KeywordFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        let bytes = input.as_bytes();
        for m in self.ac.find_iter(bytes) {
            if is_word_boundary(bytes, m.start(), m.end()) {
                let style = self.styles[m.pattern().as_usize()];
                if style.bg.is_some() {
                    collector.push_padded(m.start(), m.end(), style);
                } else {
                    collector.push(m.start(), m.end(), style);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn kw(words: &[&str], style: Style) -> KeywordConfig {
        KeywordConfig {
            words: words.iter().map(ToString::to_string).collect(),
            style,
        }
    }

    #[test]
    fn finds_keywords() {
        let finder = KeywordFinder::new(&[kw(&["null", "true", "false"], Style::new().fg(Color::Red))]).unwrap();
        let texts = super::super::span_texts("value is null or true", &finder);
        assert_eq!(texts, ["null", "true"]);
    }

    #[test]
    fn respects_word_boundaries() {
        let finder = KeywordFinder::new(&[kw(&["null"], Style::new().fg(Color::Red))]).unwrap();
        let mut collector = Collector::new();
        finder.find_spans("nullable is not null", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert_eq!(&"nullable is not null"[spans[0].start..spans[0].end], "null");
    }

    #[test]
    fn longer_keyword_wins_over_its_prefix() {
        let finder = KeywordFinder::new(&[kw(&["WARN", "WARNING"], Style::new().fg(Color::Yellow))]).unwrap();
        let texts = super::super::span_texts("level WARNING here", &finder);
        assert_eq!(texts, ["WARNING"]);
    }

    #[test]
    fn each_keyword_keeps_the_style_of_its_config() {
        let red = Style::new().fg(Color::Red);
        let green = Style::new().fg(Color::Green);
        let finder = KeywordFinder::new(&[kw(&["ERROR"], red), kw(&["SUCCESS"], green)]).unwrap();

        let mut collector = Collector::new();
        finder.find_spans("ERROR then SUCCESS", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].style, red);
        assert_eq!(spans[1].style, green);
    }

    #[test]
    fn background_style_marks_span_padded() {
        let finder = KeywordFinder::new(&[kw(&["ERROR"], Style::new().on(Color::Red))]).unwrap();
        let mut collector = Collector::new();
        finder.find_spans("level ERROR here", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert!(spans[0].padded);
    }

    #[test]
    fn foreground_only_leaves_span_unpadded() {
        let finder = KeywordFinder::new(&[kw(&["ERROR"], Style::new().fg(Color::Red))]).unwrap();
        let mut collector = Collector::new();
        finder.find_spans("level ERROR here", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert!(!spans[0].padded);
    }

    #[test]
    fn mixed_padded_and_plain_configs() {
        let badge = Style::new().on(Color::Red);
        let plain = Style::new().fg(Color::Green);
        let finder = KeywordFinder::new(&[kw(&["ERROR"], badge), kw(&["ok"], plain)]).unwrap();

        let mut collector = Collector::new();
        finder.find_spans("ERROR but ok", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert!(spans[0].padded);
        assert!(!spans[1].padded);
    }
}
