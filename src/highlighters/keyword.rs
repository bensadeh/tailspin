use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::Highlight;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashMap;

pub struct KeywordHighlighter {
    keyword: String,
    color: String,
}

impl KeywordHighlighter {
    pub fn new(keyword: String, style: &Style) -> Self {
        Self {
            keyword,
            color: to_ansi(style),
        }
    }
}

impl Highlight for KeywordHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        let keywords = KEYWORDS
            .get()
            .expect("KEYWORDS should have been initialized");
        let keyword_regex = keywords
            .get(&self.keyword)
            .expect("Keyword regex not found");

        highlight_keywords(&self.keyword, &self.color, input, keyword_regex)
    }
}

static KEYWORDS: OnceCell<HashMap<String, Regex>> = OnceCell::new();

pub fn init_keywords(keywords: Vec<String>) {
    let mut map = HashMap::new();
    for keyword in keywords {
        let escaped = regex::escape(&keyword);
        let regex = Regex::new(&format!(r"\b{}\b", escaped)).expect("Invalid regex pattern");
        map.insert(keyword, regex);
    }
    KEYWORDS
        .set(map)
        .expect("KEYWORDS should not have been initialized before");
}

fn highlight_keywords(_keyword: &str, color: &str, input: &str, keyword_regex: &Regex) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, keyword_regex)
}
