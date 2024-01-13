use crate::highlight_utils::highlight_with_awareness;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
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
});

pub struct UuidHighlighter {
    segment: Style,
    separator: Style,
}

impl UuidHighlighter {
    pub fn new(segment: Style, separator: Style) -> Self {
        Self { segment, separator }
    }
}

impl Highlight for UuidHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.dashes < 4 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_with_awareness(input, &UUID_REGEX, |caps: &Captures<'_>| {
            let mut output = String::new();
            for i in 1..caps.len() {
                if &caps[i] == "-" {
                    output.push_str(&format!("{}", self.separator.paint(&caps[i])));
                } else {
                    output.push_str(&format!("{}", self.segment.paint(&caps[i])));
                }
            }
            output
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::color::Fg;
//     use crate::theme::Style;
//
//     #[test]
//     fn test_highlight_uuids() {
//         let uuid = "550e8400-e29b-41d4-a716-446655440000";
//         let segment = Style {
//             fg: Fg::Red,
//             ..Default::default()
//         };
//         let separator = Style {
//             fg: Fg::Green,
//             ..Default::default()
//         };
//
//         let highlighter = UuidHighlighter::new(&segment, &separator);
//         let highlighted = highlighter.apply(uuid);
//
//         let segment_color = "\x1b[31m"; // ANSI color code for red
//         let separator_color = "\x1b[32m"; // ANSI color code for green
//         let expected = format!(
//             "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
//             segment_color,
//             "550e8400",
//             color::RESET,
//             separator_color,
//             "-",
//             color::RESET,
//             segment_color,
//             "e29b",
//             color::RESET,
//             separator_color,
//             "-",
//             color::RESET,
//             segment_color,
//             "41d4",
//             color::RESET,
//             separator_color,
//             "-",
//             color::RESET,
//             segment_color,
//             "a716",
//             color::RESET,
//             separator_color,
//             "-",
//             color::RESET,
//             segment_color,
//             "446655440000",
//             color::RESET
//         );
//         assert_eq!(highlighted, expected);
//     }
//
//     #[test]
//     fn test_highlight_uuids_no_uuid() {
//         let text = "this is a test string with no uuid";
//         let segment = Style {
//             fg: Fg::Red,
//             ..Default::default()
//         };
//         let separator = Style {
//             fg: Fg::Green,
//             ..Default::default()
//         };
//
//         let highlighter = UuidHighlighter::new(&segment, &separator);
//         let highlighted = highlighter.apply(text);
//
//         assert_eq!(highlighted, text);
//     }
// }
