use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlight_utils;
use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str| -> String { highlight_numbers(&color, input) })
}

fn highlight_numbers(color: &str, input: &str) -> String {
    let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

    highlight_utils::highlight_with_awareness(color, input, &number_regex)
}
