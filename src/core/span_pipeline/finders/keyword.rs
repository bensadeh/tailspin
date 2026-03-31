use aho_corasick::{AhoCorasick, AhoCorasickBuilder, BuildError, MatchKind};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct KeywordFinder {
    ac: AhoCorasick,
    style: Style,
    has_background: bool,
}

impl KeywordFinder {
    pub fn new(words: &[impl AsRef<[u8]>], style: Style) -> Result<Self, BuildError> {
        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(words)?;

        let has_background = style.bg.is_some();

        Ok(Self {
            ac,
            style,
            has_background,
        })
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
                if self.has_background {
                    collector.push_padded(m.start(), m.end(), self.style);
                } else {
                    collector.push(m.start(), m.end(), self.style);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn finds_keywords() {
        let finder = KeywordFinder::new(&["null", "true", "false"], Style::new().fg(Color::Red)).unwrap();
        let mut collector = Collector::new(0);
        finder.find_spans("value is null or true", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert_eq!(&"value is null or true"[spans[0].start..spans[0].end], "null");
        assert_eq!(&"value is null or true"[spans[1].start..spans[1].end], "true");
    }

    #[test]
    fn respects_word_boundaries() {
        let finder = KeywordFinder::new(&["null"], Style::new().fg(Color::Red)).unwrap();
        let mut collector = Collector::new(0);
        finder.find_spans("nullable is not null", &mut collector);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert_eq!(&"nullable is not null"[spans[0].start..spans[0].end], "null");
    }

    #[test]
    fn background_style_produces_padded_ranges() {
        let finder = KeywordFinder::new(&["ERROR"], Style::new().on(Color::Red)).unwrap();
        let mut collector = Collector::new(0);
        finder.find_spans("level ERROR here", &mut collector);

        let (spans, padded) = collector.into_parts();
        assert_eq!(spans.len(), 1);
        assert_eq!(padded.len(), 1);
        assert_eq!(padded[0], spans[0].start..spans[0].end);
    }

    #[test]
    fn foreground_only_produces_no_padded_ranges() {
        let finder = KeywordFinder::new(&["ERROR"], Style::new().fg(Color::Red)).unwrap();
        let mut collector = Collector::new(0);
        finder.find_spans("level ERROR here", &mut collector);

        let (spans, padded) = collector.into_parts();
        assert_eq!(spans.len(), 1);
        assert!(padded.is_empty());
    }
}
