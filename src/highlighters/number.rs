use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::regexes::NUMBER_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct NumberHighlighter {
    color: String,
}

impl NumberHighlighter {
    pub fn new(style: &Style) -> Self {
        Self {
            color: to_ansi(style),
        }
    }
}

impl Highlight for NumberHighlighter {
    fn apply(&self, input: &str, line_info: &LineInfo) -> String {
        highlight_numbers(&self.color, input, line_info)
    }
}

fn highlight_numbers(color: &str, input: &str, line_info: &LineInfo) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, &NUMBER_REGEX)
}
