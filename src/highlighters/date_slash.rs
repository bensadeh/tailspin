use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<year>20\d{2})(?P<separator1>/)(?P<month>(0[1-9]|1[0-2]))(?P<separator2>/)(?P<day>(0[1-9]|[12][0-9]|3[01]))")
        .expect("Invalid regex pattern")
});

pub struct DateSlashHighlighter {
    number: Style,
    separator: Style,
}

impl DateSlashHighlighter {
    pub const fn new(number: Style, separator: Style) -> Self {
        Self { number, separator }
    }
}

impl Highlight for DateSlashHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.slashes < 2
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        DATE_REGEX.replace_all(input, |caps: &Captures<'_>| {
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
    }
}
