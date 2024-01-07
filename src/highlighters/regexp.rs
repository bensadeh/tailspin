use crate::color::to_ansi;
use crate::highlight_utils::generic_process_with_awareness;
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::Highlight;
use crate::{color, highlight_utils};
use regex::{Captures, Regex};

pub struct RegexpHighlighter {
    keyword_regex: Regex,
    color: String,
    border: bool,
}

impl RegexpHighlighter {
    pub fn new(regular_expression: &String, style: &Style, border: bool) -> Self {
        let keyword_regex = Regex::new(&regular_expression.to_string()).expect("Invalid regex pattern");

        Self {
            keyword_regex,
            color: to_ansi(style),
            border,
        }
    }
}

impl Highlight for RegexpHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_text_new(&self.keyword_regex, input, &self.color)
    }
}

pub(crate) fn highlight_text_new(regex: &Regex, text: &str, color: &str) -> String {
    let capture_groups = regex.captures_len() - 1;

    generic_process_with_awareness(text, |chunk| match capture_groups {
        1 => {
            let mut new_string = String::new();
            let mut last_end = 0;

            for caps in regex.captures_iter(chunk) {
                if let Some(entire_match) = caps.get(0) {
                    // Add the text before the entire regex match
                    new_string.push_str(&chunk[last_end..entire_match.start()]);

                    // Add the text from the start of the regex match to the start of the capturing group
                    new_string.push_str(&chunk[entire_match.start()..caps.get(1).unwrap().start()]);

                    // Highlight the captured group
                    new_string.push_str(color);
                    new_string.push_str(caps.get(1).unwrap().as_str());
                    new_string.push_str(color::RESET);

                    // Add the text from the end of the capturing group to the end of the entire regex match
                    new_string.push_str(&chunk[caps.get(1).unwrap().end()..entire_match.end()]);

                    // Update the end of the last entire regex match
                    last_end = entire_match.end();
                }
            }

            // Add the remaining text after the last regex match
            new_string.push_str(&chunk[last_end..]);
            new_string
        }
        _ => regex
            .replace_all(chunk, |caps: &Captures| {
                format!("{}{}{}", color, &caps[0], color::RESET)
            })
            .to_string(),
    })
}
