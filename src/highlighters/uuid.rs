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
        line_info.dashes < 4
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        UUID_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
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
            .to_string()
    }
}
