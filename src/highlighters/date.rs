use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{4}-\d{2}-\d{2}").expect("Invalid regex pattern"));

pub struct DateHighlighter {
    style: Style,
}

impl DateHighlighter {
    pub fn new(style: Style) -> Self {
        Self { style }
    }
}

impl Highlight for DateHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.dashes < 2
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
