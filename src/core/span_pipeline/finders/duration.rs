use super::build_regex;
use regex::Regex;

use crate::core::config::DurationConfig;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct DurationFinder {
    regex: Regex,
    value: StyleId,
    separator: StyleId,
    unit: StyleId,
}

impl DurationFinder {
    pub fn new(config: DurationConfig, palette: &mut Palette) -> Self {
        let pattern = r"(?x)
            \b
            (?P<integer>\d+)                    # integer part
            (?:(?P<dot>\.)(?P<fraction>\d+))?   # optional fractional part
            (?P<unit>ns|us|ms|s|m|h)            # time unit
            \b
        ";

        let regex = build_regex(pattern);

        Self {
            regex,
            value: palette.intern(config.value),
            separator: palette.intern(config.separator),
            unit: palette.intern(config.unit),
        }
    }
}

impl Finder for DurationFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        // Every match contains a digit immediately followed by its unit's first letter.
        let digit_then_unit = |w: &[u8]| w[0].is_ascii_digit() && matches!(w[1], b'n' | b'u' | b'm' | b's' | b'h');
        if !input.as_bytes().windows(2).any(digit_then_unit) {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            let integer = caps.name("integer").unwrap();
            collector.push(integer.start(), integer.end(), self.value);

            if let Some(dot) = caps.name("dot") {
                let fraction = caps.name("fraction").unwrap();
                collector.push(dot.start(), dot.end(), self.separator);
                collector.push(fraction.start(), fraction.end(), self.value);
            }

            let unit = caps.name("unit").unwrap();
            collector.push(unit.start(), unit.end(), self.unit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::span_texts;
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> DurationFinder {
        DurationFinder::new(
            DurationConfig {
                value: Style::new().fg(Color::Cyan),
                separator: Style::new().fg(Color::Red),
                unit: Style::new().fg(Color::Magenta),
            },
            &mut Palette::new(),
        )
    }

    #[test]
    fn finds_durations() {
        let texts = span_texts("took 150ms and 2.5s", &make_finder());
        assert_eq!(texts, ["150", "ms", "2", ".", "5", "s"]);
    }

    #[test]
    fn finds_every_unit() {
        let texts = span_texts("5ns 5us 5ms 5s 5m 5h", &make_finder());
        assert_eq!(texts, ["5", "ns", "5", "us", "5", "ms", "5", "s", "5", "m", "5", "h"]);
    }

    #[test]
    fn plain_numbers_are_not_durations() {
        assert!(span_texts("status 200 port 8080", &make_finder()).is_empty());
    }

    #[test]
    fn compound_durations_do_not_match() {
        assert!(span_texts("waited 1h30m", &make_finder()).is_empty());
    }

    #[test]
    fn unit_must_terminate_the_word() {
        assert!(span_texts("5management 3msg", &make_finder()).is_empty());
    }
}
