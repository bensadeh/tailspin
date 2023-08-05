use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::HighlightFn;
use lazy_static::lazy_static;
use regex::Regex;

pub fn highlight(style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_numbers(&color, input, line_info, &NUMBER_REGEX)
    })
}

lazy_static! {
    static ref NUMBER_REGEX: Regex = {
        Regex::new(
            r"(?x)       # Enable comments and whitespace insensitivity
            \b           # Word boundary, ensures we are at the start of a number
            \d+          # Matches one or more digits
            (\.          # Start a group to match a decimal part
            \d+          # Matches one or more digits after the dot
            )?           # The decimal part is optional
            \b           # Word boundary, ensures we are at the end of a number
            ",
        )
        .expect("Invalid regex pattern")
    };
}

fn highlight_numbers(
    color: &str,
    input: &str,
    _line_info: &LineInfo,
    number_regex: &Regex,
) -> String {
    highlight_utils::highlight_with_awareness_replace_all(color, input, number_regex)
}
