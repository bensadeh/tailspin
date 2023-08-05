use crate::color;
use crate::color::to_ansi;
use crate::highlight_utils::highlight_with_awareness;
use crate::line_info::LineInfo;
use crate::regexes::UUID_REGEX;
use crate::theme::Style;
use crate::types::Highlight;
use regex::{Captures, Regex};

pub struct UuidHighlighter {
    segment_color: String,
    separator_color: String,
}

impl UuidHighlighter {
    pub fn new(segment: &Style, separator: &Style) -> Self {
        Self {
            segment_color: to_ansi(segment),
            separator_color: to_ansi(separator),
        }
    }
}

impl Highlight for UuidHighlighter {
    fn apply(&self, input: &str, line_info: &LineInfo) -> String {
        highlight_uuids(
            &self.segment_color,
            &self.separator_color,
            input,
            line_info,
            &UUID_REGEX,
        )
    }
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
    use crate::color::Fg;
    use crate::theme::Style;

    #[test]
    fn test_highlight_uuids() {
        let line_info = LineInfo {
            dashes: 4,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colons: 0,
        };

        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        let segment = Style {
            fg: Fg::Red,
            ..Default::default()
        };
        let separator = Style {
            fg: Fg::Green,
            ..Default::default()
        };

        let highlighter = UuidHighlighter::new(&segment, &separator);
        let highlighted = highlighter.apply(uuid, &line_info);

        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green
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
        let line_info = LineInfo {
            dashes: 4,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colons: 0,
        };

        let text = "this is a test string with no uuid";
        let segment = Style {
            fg: Fg::Red,
            ..Default::default()
        };
        let separator = Style {
            fg: Fg::Green,
            ..Default::default()
        };

        let highlighter = UuidHighlighter::new(&segment, &separator);
        let highlighted = highlighter.apply(text, &line_info);

        // The input string does not contain a UUID, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
