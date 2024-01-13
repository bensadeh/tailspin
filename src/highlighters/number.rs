use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)       # Enable comments and whitespace insensitivity
            \b       # Word boundary, ensures we are at the start of a number
            \d+      # Matches one or more digits
            (\.      # Start a group to match a decimal part
            \d+      # Matches one or more digits after the dot
            )?       # The decimal part is optional
            \b       # Word boundary, ensures we are at the end of a number
            ",
    )
    .expect("Invalid regex pattern")
});

pub struct NumberHighlighter {
    style: Style,
}

impl NumberHighlighter {
    pub fn new(style: Style) -> Self {
        Self { style }
    }
}

impl Highlight for NumberHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        let color = &self.style;
        highlight_utils::highlight_with_awareness_replace_all_with_new_style(color, input, &NUMBER_REGEX, false)
    }
}
