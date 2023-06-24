use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlight_utils;
use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(keyword: String, style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str| -> String { highlight_keywords(&keyword, &color, input) })
}

fn highlight_keywords(keyword: &str, color: &str, input: &str) -> String {
    let keyword = regex::escape(keyword);
    let keyword_regex = Regex::new(&format!(r"\b{}\b", keyword)).expect("Invalid regex pattern");

    highlight_utils::highlight_with_awareness(color, input, &keyword_regex)
}
