use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"20\d{2}/(0[1-9]|1[0-2])/(0[1-9]|[12][0-9]|3[01])").expect("Invalid regex pattern"));

pub struct DateSlashHighlighter {
    style: Style,
}

impl DateSlashHighlighter {
    pub fn new(style: Style) -> Self {
        Self { style }
    }
}

impl Highlight for DateSlashHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.slashes < 2
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        DATE_REGEX
            .replace_all(input, |caps: &Captures<'_>| format!("{}", self.style.paint(&caps[0])))
            .to_string()
    }
}
