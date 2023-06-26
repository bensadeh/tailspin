use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlight_utils;
use crate::highlighters::HighlightFn;
use crate::line_info::LineInfo;
use regex::Regex;

pub fn highlight(keyword: String, style: &Style) -> HighlightFn {
    let color = to_ansi(style);
    let keyword = regex::escape(&keyword);
    let keyword_regex = Regex::new(&format!(r"\b{}\b", keyword)).expect("Invalid regex pattern");

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_keywords(&keyword, &color, input, line_info, &keyword_regex)
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
