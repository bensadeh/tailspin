use crate::color::{to_ansi, RESET};
use crate::line_info::LineInfo;
use crate::regex::PROCESS_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct ProcessHighlighter {
    process_name_color: String,
    bracket_color: String,
    process_num_color: String,
}

impl ProcessHighlighter {
    pub fn new(process_name_style: &Style, bracket_style: &Style, process_num_style: &Style) -> Self {
        Self {
            process_name_color: to_ansi(process_name_style),
            bracket_color: to_ansi(bracket_style),
            process_num_color: to_ansi(process_num_style),
        }
    }
}

impl Highlight for ProcessHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.left_bracket < 1 || line_info.right_bracket < 1 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_processes(
            &self.process_name_color,
            &self.bracket_color,
            &self.process_num_color,
            input,
        )
    }
}

fn highlight_processes(process_name_color: &str, bracket_color: &str, process_num_color: &str, input: &str) -> String {
    PROCESS_REGEX
        .replace_all(input, |captures: &regex::Captures| {
            let process_name = captures
                .name("process_name")
                .map(|p| format!("{}{}", process_name_color, p.as_str()))
                .unwrap_or_default();
            let process_num = captures
                .name("process_num")
                .map(|n| format!("{}{}", process_num_color, n.as_str()))
                .unwrap_or_default();

            format!(
                "{}{}{}[{}{}{}]{}",
                process_name, RESET, bracket_color, process_num, RESET, bracket_color, RESET
            )
        })
        .to_string()
}
