use memchr::memchr2;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct DateDashFinder {
    regex: Regex,
    date: Style,
    separator: Style,
}

impl DateDashFinder {
    pub fn new(date: Style, separator: Style) -> Self {
        // Both branches are exactly 10 bytes (4+1+2+1+2), so we can use
        // find_iter and compute component offsets arithmetically.
        let pattern = r"(?x)
            # Branch A: YYYY-xx-xx
            (?: (?: 19\d{2} | 20\d{2} ) [-/] (?: 0[1-9] | [12]\d | 3[01] ) [-/] (?: 0[1-9] | [12]\d | 3[01] ) )
            |
            # Branch B: xx-xx-YYYY
            (?: (?: 0[1-9] | [12]\d | 3[01] ) [-/] (?: 0[1-9] | [12]\d | 3[01] ) [-/] (?: 19\d{2} | 20\d{2} ) )
        ";

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded date-dash regex must compile");

        Self { regex, date, separator }
    }
}

impl Finder for DateDashFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr2(b'-', b'/', input.as_bytes()).is_none() {
            return;
        }

        for m in self.regex.find_iter(input) {
            let s = m.start();
            let bytes = m.as_str().as_bytes();

            // Both branches are exactly 10 bytes. Distinguish by checking
            // whether position 4 is a separator (Branch A: YYYY-MM-DD)
            // or position 2 is a separator (Branch B: MM-DD-YYYY).
            if bytes[4] == b'-' || bytes[4] == b'/' {
                // Branch A: YYYY-MM-DD
                collector.push(s, s + 4, self.date);
                collector.push(s + 4, s + 5, self.separator);
                collector.push(s + 5, s + 7, self.date);
                collector.push(s + 7, s + 8, self.separator);
                collector.push(s + 8, s + 10, self.date);
            } else {
                // Branch B: MM-DD-YYYY
                collector.push(s, s + 2, self.date);
                collector.push(s + 2, s + 3, self.separator);
                collector.push(s + 3, s + 5, self.date);
                collector.push(s + 5, s + 6, self.separator);
                collector.push(s + 6, s + 10, self.date);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> DateDashFinder {
        DateDashFinder::new(Style::new().fg(Color::Magenta), Style::new().fg(Color::Blue))
    }

    fn span_texts<'a>(input: &'a str, finder: &DateDashFinder) -> Vec<&'a str> {
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
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
}
