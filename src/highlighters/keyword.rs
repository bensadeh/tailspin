use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use regex::Regex;

pub struct KeywordHighlighter {
    keyword_regex: Regex,
    style: Style,
    border: bool,
}

impl KeywordHighlighter {
    pub fn new(keywords: Vec<String>, style: Style, border: bool) -> Self {
        let keyword_pattern = keywords
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>()
            .join("|");

        let keyword_regex = Regex::new(&format!(r"\b({})\b", keyword_pattern)).expect("Invalid regex pattern");

        Self {
            keyword_regex,
            style,
            border,
        }
    }
}

impl Highlight for KeywordHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_utils::highlight_with_awareness_replace_all_with_new_style(
            &self.style,
            input,
            &self.keyword_regex,
            self.border,
        )
    }
}
