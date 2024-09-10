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
    pub const fn new(number: Style, separator: Style) -> Self {
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
                let year = &caps["year"];
                let month = &caps["month"];
                let day = &caps["day"];
                let separator1 = &caps["separator1"];
                let separator2 = &caps["separator2"];

                format!(
                    "{}{}{}{}{}",
                    self.number.paint(year),
                    self.separator.paint(separator1),
                    self.number.paint(month),
                    self.separator.paint(separator2),
                    self.number.paint(day)
                )
            })
            .to_string()
    }
}
