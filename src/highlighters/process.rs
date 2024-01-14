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
    pub fn new(process_name: Style, bracket: Style, process_num: Style) -> Self {
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

    fn apply(&self, input: &str) -> String {
        PROCESS_REGEX
            .replace_all(input, |captures: &regex::Captures| {
                let process_name = captures
                    .name("process_name")
                    .map(|p| format!("{}", self.process_name.paint(p.as_str())))
                    .unwrap_or_default();
                let process_num = captures
                    .name("process_num")
                    .map(|n| format!("{}", self.process_num.paint(n.as_str())))
                    .unwrap_or_default();

                format!(
                    "{}{}{}{}",
                    process_name,
                    self.bracket.paint("["),
                    process_num,
                    self.bracket.paint("]")
                )
            })
            .to_string()
    }
}
