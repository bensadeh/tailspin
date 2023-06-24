use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str| -> String { highlight_dates(&color, input) })
}

fn highlight_dates(color: &str, input: &str) -> String {
    let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

    let highlighted = number_regex.replace_all(input, |caps: &regex::Captures<'_>| {
        format!("{}{}{}", color, &caps[0], color::RESET)
    });

    highlighted.into_owned()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_highlight_dates() {
//         let input = "The date is 2023-06-24.";
//         let expected_output = format!("The date is {}2023-06-24{}.", color::RED, color::RESET);
//         let color = color::RED;
//         assert_eq!(highlight_dates(&color, &input), expected_output);
//     }
// }
