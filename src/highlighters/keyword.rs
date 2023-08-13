use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::Highlight;
use regex::Regex;

pub struct KeywordHighlighter {
    keyword_regex: Regex,
    color: String,
}

impl KeywordHighlighter {
    pub fn new(keywords: &[String], style: &Style) -> Self {
        let keyword_pattern = keywords
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>()
            .join("|");

        let keyword_regex =
            Regex::new(&format!(r"\b({})\b", keyword_pattern)).expect("Invalid regex pattern");

        Self {
            keyword_regex,
            color: to_ansi(style),
        }
    }
}

impl Highlight for KeywordHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_keywords(&self.color, input, &self.keyword_regex)
    }
}

fn highlight_keywords(color: &str, input: &str, keyword_regex: &Regex) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, keyword_regex)
}
