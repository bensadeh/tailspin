use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use regex::Regex;

pub struct RegexpHighlighter {
    keyword_regex: Regex,
    style: Style,
    _border: bool,
}

impl RegexpHighlighter {
    pub fn new(regular_expression: &str, style: Style, border: bool) -> Self {
        let keyword_regex = Regex::new(regular_expression).expect("Invalid regex pattern");

        Self {
            keyword_regex,
            style,
            _border: border,
        }
    }
}

impl Highlight for RegexpHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        let regex = &self.keyword_regex;
        let color = &self.style;
        let capture_groups = regex.captures_len() - 1;

        let mut new_string = String::new();
        let mut last_end = 0;

        for caps in regex.captures_iter(input) {
            if let Some(entire_match) = caps.get(0) {
                // Add the text before the regex match
                new_string.push_str(&get_pre_match_text(input, last_end, entire_match.start()));

                // Determine what part of the match to highlight
                if capture_groups == 1 {
                    if let Some(captured) = caps.get(1) {
                        // Add the text before the capturing group (from the start of the entire match)
                        new_string.push_str(&get_pre_match_text(input, entire_match.start(), captured.start()));
                        // Highlight the captured group
                        new_string.push_str(&format!("{}", color.paint(captured.as_str())));
                        // Add the text after the capturing group (up to the end of the entire match)
                        new_string.push_str(&get_pre_match_text(input, captured.end(), entire_match.end()));
                    }
                } else {
                    // No capturing groups or more than one, highlight the entire match
                    let captured = entire_match.as_str();
                    new_string.push_str(&format!("{}", color.paint(captured)));
                }

                // Update the last_end position to the end of the entire match
                last_end = entire_match.end();
            }
        }

        // Add the remaining text after the last match
        new_string.push_str(&input[last_end..]);
        new_string
    }
}

// Extract text before the regex match
fn get_pre_match_text(text: &str, start: usize, end: usize) -> String {
    text[start..end].to_string()
}
