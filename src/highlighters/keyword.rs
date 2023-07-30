use crate::color::to_ansi;
use crate::highlight_utils;
use crate::highlighters::HighlightFn;
use crate::line_info::LineInfo;
use crate::theme::Style;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashMap;

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

pub fn highlight(keyword: String, style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        let keywords = KEYWORDS
            .get()
            .expect("KEYWORDS should have been initialized");
        let keyword_regex = keywords.get(&keyword).expect("Keyword regex not found");

        highlight_keywords(&keyword, &color, input, line_info, keyword_regex)
    })
}

fn highlight_keywords(
    _keyword: &str,
    color: &str,
    input: &str,
    _line_info: &LineInfo,
    keyword_regex: &Regex,
) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, keyword_regex)
}
