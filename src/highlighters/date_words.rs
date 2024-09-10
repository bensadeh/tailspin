use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_WORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?x)
        (?P<day1>\b(?:Mon|Tue|Wed|Thu|Fri|Sat|Sun)\b)?
        \s*
        (?P<month>\b(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\b)
        \s+
        (?P<day2>\b(?:[0-2]?[0-9]|3[0-1])\b)
    "#,
    )
    .expect("Invalid regex pattern")
});

pub struct DateWordHighlighter {
    day_name: Style,
    month_name: Style,
    day_number: Style,
}

impl DateWordHighlighter {
    pub const fn new(day_name: Style, month_name: Style, day_number: Style) -> Self {
        Self {
            day_name,
            month_name,
            day_number,
        }
    }
}

impl Highlight for DateWordHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        DATE_WORD_REGEX.replace_all(input, |caps: &Captures<'_>| {
            let day1 = caps.name("day1").map(|m| m.as_str()).unwrap_or_default();
            let month = &caps["month"];
            let day2 = &caps["day2"];

            format!(
                "{}{} {}",
                self.day_name.paint(day1),
                self.month_name.paint(month),
                self.day_number.paint(day2)
            )
        })
    }
}
