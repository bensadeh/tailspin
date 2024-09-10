use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

// Keep the original regex that matches the whole UUID
static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
            \b[0-9a-fA-F]{8}\b    # Match first segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match second segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match third segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match fourth segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{12}\b   # Match last segment of UUID
        ",
    )
    .expect("Invalid UUID regex pattern")
});

pub struct UuidHighlighter {
    number: Style,
    letter: Style,
    dash: Style,
}

impl UuidHighlighter {
    pub const fn new(number_style: Style, letter_style: Style, dash_style: Style) -> Self {
        Self {
            number: number_style,
            letter: letter_style,
            dash: dash_style,
        }
    }
}

impl Highlight for UuidHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.dashes < 4
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        UUID_REGEX.replace_all(input, |caps: &Captures<'_>| {
            caps[0]
                .chars()
                .map(|c| match c {
                    '0'..='9' => format!("{}", self.number.paint(c.to_string())),
                    'a'..='f' | 'A'..='F' => format!("{}", self.letter.paint(c.to_string())),
                    '-' => format!("{}", self.dash.paint(c.to_string())),
                    _ => c.to_string(),
                })
                .collect::<String>()
        })
    }
}
