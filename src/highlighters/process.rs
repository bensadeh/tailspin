use std::borrow::Cow;

use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;
static PROCESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<process_name>\([^)]+\)|[\w-]+)\[(?P<process_num>\d+)]").expect("Invalid regex pattern")
});

pub struct ProcessHighlighter {
    process_name: Style,
    bracket: Style,
    process_num: Style,
}

impl ProcessHighlighter {
    pub const fn new(process_name: Style, bracket: Style, process_num: Style) -> Self {
        Self {
            process_name,
            bracket,
            process_num,
        }
    }
}

impl Highlight for ProcessHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.left_bracket < 1 || line_info.right_bracket < 1
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        PROCESS_REGEX.replace_all(input, |captures: &regex::Captures| {
            format!(
                "{}{}{}{}",
                self.process_name.paint(&captures["process_name"]),
                self.bracket.paint("["),
                self.process_num.paint(&captures["process_num"]),
                self.bracket.paint("]")
            )
        })
    }
}
