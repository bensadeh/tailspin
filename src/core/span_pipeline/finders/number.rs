use super::build_regex;
use regex::Regex;

use crate::core::config::NumberConfig;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct NumberFinder {
    regex: Regex,
    style: StyleId,
}

impl NumberFinder {
    pub fn new(config: NumberConfig, palette: &mut Palette) -> Self {
        let pattern = r"(?x)
            \b          # start of number
            \d+         # integer part
            (?:\.\d+)?  # optional fractional
            \b          # end of number
        ";

        let regex = build_regex(pattern);

        Self {
            regex,
            style: palette.intern(config.style),
        }
    }
}

impl Finder for NumberFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if !input.bytes().any(|b| b.is_ascii_digit()) {
            return;
        }

        for m in self.regex.find_iter(input) {
            collector.push(m.start(), m.end(), self.style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::span_texts;
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> NumberFinder {
        NumberFinder::new(
            NumberConfig {
                style: Style::new().fg(Color::Cyan),
            },
            &mut Palette::new(),
        )
    }

    #[test]
    fn finds_numbers() {
        let texts = span_texts("hello 42 world 3.14", &make_finder());
        assert_eq!(texts, ["42", "3.14"]);
    }

    #[test]
    fn no_match_produces_no_spans() {
        assert!(span_texts("no numbers here", &make_finder()).is_empty());
    }
}
