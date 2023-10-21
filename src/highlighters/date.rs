use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::regex::DATE_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct DateHighlighter {
    style: String,
}

impl DateHighlighter {
    pub fn new(style: &Style) -> Self {
        Self { style: to_ansi(style) }
    }
}

impl Highlight for DateHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.dashes < 2 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_dates(&self.style, input)
    }
}

fn highlight_dates(style: &str, input: &str) -> String {
    highlight_utils::highlight_with_awareness_replace_all(style, input, &DATE_REGEX)
}
