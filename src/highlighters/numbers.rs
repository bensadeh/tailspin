use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(color: String) -> HighlightFn {
    Box::new(move |input: &str| -> String { highlight_numbers(&color, input) })
}

fn highlight_numbers(color: &str, input: &str) -> String {
    const RESET: &str = "\x1b[0m";

    let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

    let highlighted = number_regex.replace_all(input, |caps: &regex::Captures<'_>| {
        format!("{}{}{}", color, &caps[0], RESET)
    });

    highlighted.into_owned()
}
