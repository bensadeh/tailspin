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

    let mut new_string = String::new();
    let mut last_end = 0;

    for caps in regex.captures_iter(text) {
        if let Some(entire_match) = caps.get(0) {
            // Add the text before the regex match
            new_string.push_str(&get_pre_match_text(text, last_end, entire_match.start()));

            // Determine what part of the match to highlight
            match capture_groups {
                1 => {
                    if let Some(captured) = caps.get(1) {
                        // Add the text before the capturing group (from the start of the entire match)
                        new_string.push_str(&get_pre_match_text(text, entire_match.start(), captured.start()));
                        // Highlight the captured group
                        new_string.push_str(&highlight_capture(color, captured.as_str()));
                        // Add the text after the capturing group (up to the end of the entire match)
                        new_string.push_str(&get_pre_match_text(text, captured.end(), entire_match.end()));
                    }
                }
                _ => {
                    // No capturing groups or more than one, highlight the entire match
                    new_string.push_str(&highlight_capture(color, entire_match.as_str()));
                }
            }

            // Update the last_end position to the end of the entire match
            last_end = entire_match.end();
        }
    }

    // Add the remaining text after the last match
    new_string.push_str(&text[last_end..]);
    new_string
}

// Extract text before the regex match
fn get_pre_match_text(text: &str, start: usize, end: usize) -> String {
    text[start..end].to_string()
}

// Apply highlighting to the captured group
fn highlight_capture(color: &str, captured: &str) -> String {
    format!("{}{}{}", color, captured, color::RESET)
}
