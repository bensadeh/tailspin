use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct UnixProcessFinder {
    regex: Regex,
    name: Style,
    id: Style,
    bracket: Style,
}

impl UnixProcessFinder {
    pub fn new(name: Style, id: Style, bracket: Style) -> Self {
        // Match structure: name[pid] — we find '[' in the match to split the parts.
        let pattern = r"(?:\([A-Za-z0-9._ +:/-]+\)|[A-Za-z0-9_/-]+)\[\d+]";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded Unix process regex must compile");

        Self {
            regex,
            name,
            id,
            bracket,
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
            let bracket = memchr(b'[', bytes).unwrap();
            collector.push(s, s + bracket, self.name);
            collector.push(s + bracket, s + bracket + 1, self.bracket);
            collector.push(s + bracket + 1, s + bytes.len() - 1, self.id);
            collector.push(s + bytes.len() - 1, s + bytes.len(), self.bracket);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> UnixProcessFinder {
        UnixProcessFinder::new(
            Style::new().fg(Color::Magenta),
            Style::new().fg(Color::Green),
            Style::new().fg(Color::Blue),
        )
    }

    fn span_texts<'a>(input: &'a str, finder: &UnixProcessFinder) -> Vec<&'a str> {
        let mut collector = Collector::new();
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
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
