use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct DateTimeFinder {
    regex: Regex,
    idx: Idx,
    time: Style,
    zone: Style,
    separator: Style,
}

#[derive(Debug, Copy, Clone)]
struct Idx {
    t: usize,
    hours: usize,
    colon1: usize,
    minutes: usize,
    colon2: usize,
    seconds: usize,
    frac_sep: usize,
    frac_digits: usize,
    tz: usize,
}

impl DateTimeFinder {
    pub fn new(time: Style, zone: Style, separator: Style) -> Self {
        let pattern = r"(?x)
            (?P<T>[T\s])?
            (?P<hours>[01]?\d|2[0-3])(?P<colon1>:)
            (?P<minutes>[0-5]\d)(?P<colon2>:)
            (?P<seconds>[0-5]\d)
            (?P<frac_sep>[.,:])?(?P<frac_digits>\d+)?
            (?P<tz>Z)?
        ";

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded date-time regex must compile");

        let mut idx = Idx {
            t: 0,
            hours: 0,
            colon1: 0,
            minutes: 0,
            colon2: 0,
            seconds: 0,
            frac_sep: 0,
            frac_digits: 0,
            tz: 0,
        };
        for (i, name) in regex.capture_names().enumerate() {
            match name {
                Some("T") => idx.t = i,
                Some("hours") => idx.hours = i,
                Some("colon1") => idx.colon1 = i,
                Some("minutes") => idx.minutes = i,
                Some("colon2") => idx.colon2 = i,
                Some("seconds") => idx.seconds = i,
                Some("frac_sep") => idx.frac_sep = i,
                Some("frac_digits") => idx.frac_digits = i,
                Some("tz") => idx.tz = i,
                _ => {}
            }
        }

        Self {
            regex,
            idx,
            time,
            zone,
            separator,
        }
    }

    fn push_part(collector: &mut Collector, caps: &regex::Captures<'_>, i: usize, style: Style) {
        if let Some(m) = caps.get(i) {
            collector.push(m.start(), m.end(), style);
        }
    }
}

impl Finder for DateTimeFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b':', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            Self::push_part(collector, &caps, self.idx.t, self.zone);
            Self::push_part(collector, &caps, self.idx.hours, self.time);
            Self::push_part(collector, &caps, self.idx.colon1, self.separator);
            Self::push_part(collector, &caps, self.idx.minutes, self.time);
            Self::push_part(collector, &caps, self.idx.colon2, self.separator);
            Self::push_part(collector, &caps, self.idx.seconds, self.time);
            Self::push_part(collector, &caps, self.idx.frac_sep, self.separator);
            Self::push_part(collector, &caps, self.idx.frac_digits, self.time);
            Self::push_part(collector, &caps, self.idx.tz, self.zone);
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
        let mut collector = Collector::new(0);
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
