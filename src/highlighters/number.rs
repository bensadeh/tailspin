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
    let number_regex = Regex::new(
        r"(?x)   # Enable comments and whitespace insensitivity
    \b           # Word boundary, ensures we are at the start of a number
    \d+          # Matches one or more digits
    (\.          # Start a group to match a decimal part
    \d+          # Matches one or more digits after the dot
    )?           # The decimal part is optional
    \b           # Word boundary, ensures we are at the end of a number
    ",
    )
    .expect("Invalid regex pattern");

    highlight_utils::highlight_with_awareness_replace_all(color, input, &number_regex)
}
