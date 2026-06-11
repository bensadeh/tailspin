use super::build_regex;
use memchr::memchr2;
use regex::Regex;

use crate::core::config::DateTimeConfig;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct DateDashFinder {
    regex: Regex,
    config: DateTimeConfig,
}

impl DateDashFinder {
    pub fn new(config: DateTimeConfig) -> Self {
        // Both branches are exactly 10 bytes (4+1+2+1+2), so we can use
        // find_iter and compute component offsets arithmetically.
        let pattern = r"(?x)
            # Leading \b only: a trailing one would reject the `T` in ISO-8601
            # timestamps (2022-09-22T07:46:34), whose date half we highlight.
            \b
            (?:
                # Branch A: YYYY-xx-xx
                (?: (?: 19\d{2} | 20\d{2} ) [-/] (?: 0[1-9] | [12]\d | 3[01] ) [-/] (?: 0[1-9] | [12]\d | 3[01] ) )
                |
                # Branch B: xx-xx-YYYY
                (?: (?: 0[1-9] | [12]\d | 3[01] ) [-/] (?: 0[1-9] | [12]\d | 3[01] ) [-/] (?: 19\d{2} | 20\d{2} ) )
            )
        ";

        let regex = build_regex(pattern);

        Self { regex, config }
    }
}

impl Finder for DateDashFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr2(b'-', b'/', input.as_bytes()).is_none() {
            return;
        }

        let DateTimeConfig { date, separator, .. } = self.config;

        for m in self.regex.find_iter(input) {
            let s = m.start();
            let bytes = m.as_str().as_bytes();

            // Both branches are exactly 10 bytes. Distinguish by checking
            // whether position 4 is a separator (Branch A: YYYY-MM-DD)
            // or position 2 is a separator (Branch B: MM-DD-YYYY).
            if bytes[4] == b'-' || bytes[4] == b'/' {
                // Branch A: YYYY-MM-DD
                collector.push(s, s + 4, date);
                collector.push(s + 4, s + 5, separator);
                collector.push(s + 5, s + 7, date);
                collector.push(s + 7, s + 8, separator);
                collector.push(s + 8, s + 10, date);
            } else {
                // Branch B: MM-DD-YYYY
                collector.push(s, s + 2, date);
                collector.push(s + 2, s + 3, separator);
                collector.push(s + 3, s + 5, date);
                collector.push(s + 5, s + 6, separator);
                collector.push(s + 6, s + 10, date);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::span_texts;
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> DateDashFinder {
        DateDashFinder::new(DateTimeConfig {
            date: Style::new().fg(Color::Magenta),
            separator: Style::new().fg(Color::Blue),
            ..Default::default()
        })
    }

    #[test]
    fn yyyy_mm_dd_with_dashes() {
        let texts = span_texts("2022-09-09", &make_finder());
        assert_eq!(texts, ["2022", "-", "09", "-", "09"]);
    }

    #[test]
    fn yyyy_mm_dd_with_slashes() {
        let texts = span_texts("2022/12/30", &make_finder());
        assert_eq!(texts, ["2022", "/", "12", "/", "30"]);
    }

    #[test]
    fn dd_mm_yyyy_with_dashes() {
        let texts = span_texts("09-09-2022", &make_finder());
        assert_eq!(texts, ["09", "-", "09", "-", "2022"]);
    }

    #[test]
    fn dd_mm_yyyy_with_slashes() {
        let texts = span_texts("09/09/2022", &make_finder());
        assert_eq!(texts, ["09", "/", "09", "/", "2022"]);
    }

    #[test]
    fn invalid_year_no_match() {
        assert!(span_texts("3022-09-09", &make_finder()).is_empty());
    }

    #[test]
    fn invalid_month_no_match() {
        assert!(span_texts("2022-19-39", &make_finder()).is_empty());
    }

    #[test]
    fn invalid_year_branch_b_no_match() {
        assert!(span_texts("19/39/3023", &make_finder()).is_empty());
    }

    #[test]
    fn no_dates_no_match() {
        assert!(span_texts("No dates here!", &make_finder()).is_empty());
    }

    #[test]
    fn embedded_in_longer_number_no_match() {
        assert!(span_texts("12022-09-09", &make_finder()).is_empty());
    }

    #[test]
    fn date_in_iso8601_timestamp_matches() {
        let texts = span_texts("2022-09-22T07:46:34.171800155Z", &make_finder());
        assert_eq!(texts, ["2022", "-", "09", "-", "22"]);
    }
}
