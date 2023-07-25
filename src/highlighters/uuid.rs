use crate::color;
use crate::color::to_ansi;
use crate::config::Style;
use crate::highlight_utils::highlight_with_awareness;
use crate::highlighters::HighlightFn;
use crate::line_info::LineInfo;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn highlight(segment: &Style, separator: &Style) -> HighlightFn {
    let segment_color = to_ansi(segment);
    let separator_color = to_ansi(separator);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_uuids(
            &segment_color,
            &separator_color,
            input,
            line_info,
            &UUID_REGEX,
        )
    })
}

lazy_static! {
    static ref UUID_REGEX: Regex = {
        Regex::new(
            r"(?x)
            (\b[0-9a-fA-F]{8}\b)    # Match first segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{4}\b)    # Match second segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{4}\b)    # Match third segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{4}\b)    # Match fourth segment of UUID
            (-)                     # Match separator
            (\b[0-9a-fA-F]{12}\b)   # Match last segment of UUID
        ",
        )
        .expect("Invalid UUID regex pattern")
    };
}

fn highlight_uuids(
    segment_color: &str,
    separator_color: &str,
    input: &str,
    line_info: &LineInfo,
    uuid_regex: &Regex,
) -> String {
    if line_info.dashes < 4 {
        return input.to_string();
    }

    highlight_with_awareness(input, uuid_regex, |caps: &Captures<'_>| {
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
        let line_info = &LineInfo {
            dashes: 4,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colons: 0,
        };

        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted =
            highlight_uuids(segment_color, separator_color, uuid, line_info, &UUID_REGEX);

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
        let line_info = &LineInfo {
            dashes: 4,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colons: 0,
        };

        let text = "this is a test string with no uuid";
        let segment_color = "\x1b[31m";
        let separator_color = "\x1b[32m";

        let highlighted =
            highlight_uuids(segment_color, separator_color, text, line_info, &UUID_REGEX);

        // The input string does not contain a UUID, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
