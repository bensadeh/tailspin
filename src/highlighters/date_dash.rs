use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<year>\d{4})(?P<separator1>-)(?P<month>\d{2})(?P<separator2>-)(?P<day>\d{2})")
        .expect("Invalid regex pattern")
});

pub struct DateDashHighlighter {
    number: Style,
    separator: Style,
}

impl DateDashHighlighter {
    pub fn new(number: Style, separator: Style) -> Self {
        Self { number, separator }
    }
}

impl Highlight for DateDashHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.dashes < 2
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        DATE_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                let year = caps.name("year").map(|m| m.as_str());
                let month = caps.name("month").map(|m| m.as_str());
                let day = caps.name("day").map(|m| m.as_str());
                let separator1 = caps.name("separator1").map(|m| m.as_str());
                let separator2 = caps.name("separator2").map(|m| m.as_str());

                match (year, month, day, separator1, separator2) {
                    (Some(y), Some(mo), Some(d), Some(s1), Some(s2)) => format!(
                        "{}{}{}{}{}",
                        self.number.paint(y),
                        self.separator.paint(s1),
                        self.number.paint(mo),
                        self.separator.paint(s2),
                        self.number.paint(d)
                    ),
                    _ => input.to_string(),
                }
            })
            .to_string()
    }
}
