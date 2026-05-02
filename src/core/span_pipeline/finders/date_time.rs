use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct DateTimeFinder {
    regex: Regex,
    time: Style,
    zone: Style,
    separator: Style,
}

impl DateTimeFinder {
    pub fn new(time: Style, zone: Style, separator: Style) -> Self {
        // Match structure: [T| ]? H?H:MM:SS [.,,:]digits? Z?
        // We use find_iter and parse the fixed structure from match bytes.
        let pattern = r"(?x)
            [T\s]?
            (?:[01]?\d|2[0-3]):
            [0-5]\d:
            [0-5]\d
            (?:[.,:]  \d+)?
            Z?
        ";

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded date-time regex must compile");

        Self {
            regex,
            time,
            zone,
            separator,
        }
    }
}

impl Finder for DateTimeFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b':', input.as_bytes()).is_none() {
            return;
        }

        for m in self.regex.find_iter(input) {
            let s = m.start();
            let bytes = m.as_str().as_bytes();
            let mut pos = 0;

            // Optional T or whitespace prefix
            if !bytes[0].is_ascii_digit() {
                collector.push(s, s + 1, self.zone);
                pos = 1;
            }

            // Hours (1 or 2 digits) — scan to first ':'
            let colon1 = bytes[pos..].iter().position(|&b| b == b':').unwrap() + pos;
            collector.push(s + pos, s + colon1, self.time);
            collector.push(s + colon1, s + colon1 + 1, self.separator);
            pos = colon1 + 1;

            // Minutes (2 digits) + ':'
            collector.push(s + pos, s + pos + 2, self.time);
            collector.push(s + pos + 2, s + pos + 3, self.separator);
            pos += 3;

            // Seconds (2 digits)
            collector.push(s + pos, s + pos + 2, self.time);
            pos += 2;

            // Optional fractional part: separator + digits
            if pos < bytes.len() && matches!(bytes[pos], b'.' | b',' | b':') {
                collector.push(s + pos, s + pos + 1, self.separator);
                pos += 1;

                let digit_start = pos;
                while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                    pos += 1;
                }
                if pos > digit_start {
                    collector.push(s + digit_start, s + pos, self.time);
                }
            }

            // Optional Z suffix
            if pos < bytes.len() && bytes[pos] == b'Z' {
                collector.push(s + pos, s + pos + 1, self.zone);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> DateTimeFinder {
        DateTimeFinder::new(
            Style::new().fg(Color::Red),
            Style::new().fg(Color::Blue),
            Style::new().fg(Color::Yellow),
        )
    }

    fn span_texts<'a>(input: &'a str, finder: &DateTimeFinder) -> Vec<&'a str> {
        let mut collector = Collector::new();
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn basic_time() {
        let texts = span_texts("07:46:34", &make_finder());
        assert_eq!(texts, ["07", ":", "46", ":", "34"]);
    }

    #[test]
    fn time_with_dot_fractional() {
        let texts = span_texts("10:51:19.251", &make_finder());
        assert_eq!(texts, ["10", ":", "51", ":", "19", ".", "251"]);
    }

    #[test]
    fn time_with_colon_fractional() {
        let texts = span_texts("11:47:39:850", &make_finder());
        assert_eq!(texts, ["11", ":", "47", ":", "39", ":", "850"]);
    }

    #[test]
    fn single_digit_hour() {
        let texts = span_texts("3:33:30", &make_finder());
        assert_eq!(texts, ["3", ":", "33", ":", "30"]);
    }

    #[test]
    fn iso8601_with_t_and_z() {
        let texts = span_texts("2022-09-22T07:46:34.171800155Z", &make_finder());
        assert!(texts.contains(&"T"));
        assert!(texts.contains(&"07"));
        assert!(texts.contains(&"171800155"));
        assert!(texts.contains(&"Z"));
    }

    #[test]
    fn datetime_with_space_separator_and_comma_frac() {
        let texts = span_texts("2022-09-09 11:48:34,534", &make_finder());
        assert!(texts.contains(&" "));
        assert!(texts.contains(&"11"));
        assert!(texts.contains(&","));
        assert!(texts.contains(&"534"));
    }

    #[test]
    fn iso8601_with_timezone_offset() {
        let texts = span_texts("2024-09-14T07:57:30.659+02:00", &make_finder());
        assert!(texts.contains(&"T"));
        assert!(texts.contains(&"07"));
        assert!(texts.contains(&"30"));
        assert!(texts.contains(&"659"));
    }

    #[test]
    fn ipv6_should_not_match_as_time() {
        // IPv6 addresses contain colons but should not be matched by DateTime
        assert!(span_texts("2001:db8::ff00:42:8329", &make_finder()).is_empty());
    }

    #[test]
    fn no_time_no_match() {
        assert!(span_texts("No time here!", &make_finder()).is_empty());
    }
}
