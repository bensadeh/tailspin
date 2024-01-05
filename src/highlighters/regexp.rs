use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::Highlight;
use regex::Regex;

pub struct RegexpHighlighter {
    keyword_regex: Regex,
    color: String,
    border: bool,
}

impl RegexpHighlighter {
    pub fn new(regular_expression: &String, style: &Style, border: bool) -> Self {
        let keyword_regex = Regex::new(&regular_expression.to_string()).expect("Invalid regex pattern");

        Self {
            keyword_regex,
            color: to_ansi(style),
            border,
        }
    }
}

impl Highlight for RegexpHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_keywords(&self.color, input, &self.keyword_regex, self.border)
    }
}

fn highlight_keywords(color: &str, input: &str, keyword_regex: &Regex, border: bool) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, keyword_regex, border)
}
