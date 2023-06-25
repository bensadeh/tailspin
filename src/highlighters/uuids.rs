use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlight_utils::highlight_with_awareness;
use crate::highlighters::HighlightFn;
use regex::{Captures, Regex};

pub fn highlight(segment: &Style, separator: &Style) -> HighlightFn {
    let segment_color = to_ansi(segment);
    let separator_color = to_ansi(separator);

    Box::new(move |input: &str| -> String {
        highlight_uuids(&segment_color, &separator_color, input)
    })
}

fn highlight_uuids(segment_color: &str, separator_color: &str, input: &str) -> String {
    let uuid_regex = Regex::new(
        r"(\b[0-9a-fA-F]{8}\b)(-)(\b[0-9a-fA-F]{4}\b)(-)(\b[0-9a-fA-F]{4}\b)(-)(\b[0-9a-fA-F]{4}\b)(-)(\b[0-9a-fA-F]{12}\b)"
    ).expect("Invalid regex pattern");

    highlight_with_awareness(input, &uuid_regex, |caps: &Captures<'_>| {
        let mut output = String::new();
        for i in 1..caps.len() {
            if &caps[i] == "-" {
                output.push_str(&format!("{}{}{}", separator_color, &caps[i], color::RESET));
            } else {
                output.push_str(&format!("{}{}{}", segment_color, &caps[i], color::RESET));
            }
        }
        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_uuids() {
        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted = highlight_uuids(segment_color, separator_color, uuid);

        let expected = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            segment_color,
            "550e8400",
            color::RESET,
            separator_color,
            "-",
            color::RESET,
            segment_color,
            "e29b",
            color::RESET,
            separator_color,
            "-",
            color::RESET,
            segment_color,
            "41d4",
            color::RESET,
            separator_color,
            "-",
            color::RESET,
            segment_color,
            "a716",
            color::RESET,
            separator_color,
            "-",
            color::RESET,
            segment_color,
            "446655440000",
            color::RESET
        );
        assert_eq!(highlighted, expected);
    }

    #[test]
    fn test_highlight_uuids_no_uuid() {
        let text = "this is a test string with no uuid";
        let segment_color = "\x1b[31m";
        let separator_color = "\x1b[32m";

        let highlighted = highlight_uuids(segment_color, separator_color, text);

        // The input string does not contain a UUID, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
