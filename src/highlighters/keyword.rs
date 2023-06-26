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
    let keyword_regex = Regex::new(&format!(
        r"(?x)   # Enable comments and whitespace insensitivity
        \b       # Word boundary, ensures we are at the start of a keyword
        {}       # Matches the keyword, dynamically inserted
        \b       # Word boundary, ensures we are at the end of a keyword
        ",
        keyword
    ))
    .expect("Invalid regex pattern");

    highlight_utils::highlight_with_awareness_replace_all(color, input, &keyword_regex)
}
