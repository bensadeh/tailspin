use crate::highlight_utils::highlight_with_awareness;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static PATH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)                   # Enable extended mode for readability
            (?P<path>            # Capture the path segment
                [~/.][\w./-]*    # Match zero or more word characters, dots, slashes, or hyphens
                /[\w.-]*         # Match a path segment separated by a slash
            )",
    )
    .expect("Invalid regex pattern")
});

pub struct PathHighlighter {
    segment: Style,
    separator: Style,
}

impl Highlight for PathHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.slashes == 0
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        self.highlight_paths(input)
    }
}

impl PathHighlighter {
    pub fn new(segment: Style, separator: Style) -> Self {
        Self { segment, separator }
    }

    fn highlight_paths(&self, input: &str) -> String {
        highlight_with_awareness(input, &PATH_REGEX, |caps: &Captures<'_>| {
            let mut output = String::new();
            let path = &caps[0];
            let chars: Vec<_> = path.chars().collect();

            // Check if path starts with a valid character and not a double slash
            if !(chars[0] == '/' || chars[0] == '~' || (chars[0] == '.' && chars.len() > 1 && chars[1] == '/'))
                || (chars[0] == '/' && chars.len() > 1 && chars[1] == '/')
            {
                return path.to_string();
            }

            for &char in &chars {
                match char {
                    '/' => output.push_str(&format!("{}", self.separator.paint(char.to_string()))),
                    _ => output.push_str(&format!("{}", self.segment.paint(char.to_string()))),
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
//
//     #[test]
//     fn test_highlight_paths() {
//         let path = "~/Documents/../user/.";
//         let segment_style = Style {
//             fg: Fg::Red,
//             ..Default::default()
//         };
//         let separator_style = Style {
//             fg: Fg::Green,
//             ..Default::default()
//         };
//
//         let highlighter = PathHighlighter::new(&segment_style, &separator_style);
//         let highlighted = highlighter.apply(path);
//
//         let expected = path
//             .chars()
//             .map(|ch| {
//                 if ch == '/' {
//                     format!("{}{}{}", to_ansi(&separator_style), ch, color::RESET)
//                 } else {
//                     format!("{}{}{}", to_ansi(&segment_style), ch, color::RESET)
//                 }
//             })
//             .collect::<String>();
//         assert_eq!(highlighted, expected);
//     }
//
//     #[test]
//     fn test_highlight_paths_no_path() {
//         let text = "this is a test string with no path";
//         let segment_style = Style {
//             fg: Fg::Red,
//             ..Default::default()
//         };
//         let separator_style = Style {
//             fg: Fg::Green,
//             ..Default::default()
//         };
//
//         let highlighter = PathHighlighter::new(&segment_style, &separator_style);
//         let highlighted = highlighter.apply(text);
//
//         assert_eq!(highlighted, text);
//     }
// }
