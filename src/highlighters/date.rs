use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

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

    fn apply(&self, input: &str) -> String {
        highlight_utils::highlight_with_awareness_replace_all_with_new_style(&self.style, input, &DATE_REGEX, false)
    }
}
