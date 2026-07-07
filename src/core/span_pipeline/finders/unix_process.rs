use super::build_regex;
use memchr::memchr;
use regex::Regex;

use crate::core::config::UnixProcessConfig;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct UnixProcessFinder {
    regex: Regex,
    name: StyleId,
    id: StyleId,
    bracket: StyleId,
}

impl UnixProcessFinder {
    pub fn new(config: UnixProcessConfig, palette: &mut Palette) -> Self {
        // Match structure: name[pid] — we find '[' in the match to split the parts.
        let pattern = r"(?:\([A-Za-z0-9._ +:/-]+\)|[A-Za-z0-9_/-]+)\[\d+]";
        let regex = build_regex(pattern);

        Self {
            regex,
            name: palette.intern(config.name),
            id: palette.intern(config.id),
            bracket: palette.intern(config.bracket),
        }
    }
}

impl Finder for UnixProcessFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'[', input.as_bytes()).is_none() {
            return;
        }

        for m in self.regex.find_iter(input) {
            let s = m.start();
            let bytes = m.as_str().as_bytes();

            // Match structure: name[pid]
            let bracket_pos = memchr(b'[', bytes).unwrap();
            collector.push(s, s + bracket_pos, self.name);
            collector.push(s + bracket_pos, s + bracket_pos + 1, self.bracket);
            collector.push(s + bracket_pos + 1, m.end() - 1, self.id);
            collector.push(m.end() - 1, m.end(), self.bracket);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::span_texts;
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> UnixProcessFinder {
        UnixProcessFinder::new(
            UnixProcessConfig {
                name: Style::new().fg(Color::Magenta),
                id: Style::new().fg(Color::Green),
                bracket: Style::new().fg(Color::Blue),
            },
            &mut Palette::new(),
        )
    }

    #[test]
    fn simple_process() {
        let texts = span_texts("process[1]", &make_finder());
        assert_eq!(texts, ["process", "[", "1", "]"]);
    }

    #[test]
    fn process_with_slashes() {
        let texts = span_texts("postfix/postscreen[1894]: CONNECT", &make_finder());
        assert_eq!(texts, ["postfix/postscreen", "[", "1894", "]"]);
    }

    #[test]
    fn does_not_match_ip_in_brackets() {
        // [192.168.1.22] should not match — requires digits only inside brackets
        let texts = span_texts("[192.168.1.22]:12345", &make_finder());
        assert!(texts.is_empty());
    }

    #[test]
    fn no_process_no_match() {
        assert!(span_texts("No process here!", &make_finder()).is_empty());
    }
}
