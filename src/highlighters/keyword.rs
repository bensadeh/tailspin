use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(keyword: String, style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str| -> String { highlight_keyword(&keyword, &color, input) })
}

fn highlight_keyword(keyword: &str, color: &str, input: &str) -> String {
    const RESET: &str = "\x1b[0m";

    let keyword = regex::escape(keyword);
    let keyword_regex = Regex::new(&format!(r"\b{}\b", keyword)).expect("Invalid regex pattern");

    let highlighted = keyword_regex.replace_all(input, |caps: &regex::Captures<'_>| {
        format!("{}{}{}", color, &caps[0], RESET)
    });

    highlighted.into_owned()
}
