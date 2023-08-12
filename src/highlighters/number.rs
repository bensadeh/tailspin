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
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_numbers(&self.color, input)
    }
}

fn highlight_numbers(color: &str, input: &str) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, &NUMBER_REGEX)
}
