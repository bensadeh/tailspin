use crate::core::config::RegexConfig;
use crate::core::core::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex};

pub struct RegexpHighlighter {
    regex: Regex,
    style: NuStyle,
}

impl RegexpHighlighter {
    /// This constructor takes a regular expression pattern and a `Style` object,
    /// returning a `RegexpHighlighter` that will apply the specified style
    /// to any text matching the regular expression.
    ///
    /// It supports one capture group `()`. When found, it will apply the style to the captured text.
    ///
    /// (If you are just interested in highlighting a specific keyword, you can use the simpler `KeywordHighlighter`
    /// instead.)
    /// # Example
    ///
    /// Given the regular expression pattern `'Started (.*)\.'`, the highlighter will
    /// apply the style to any text that matches the pattern within the capture group.
    /// For example, in the text `'Started process.'`, only the word `'process'` will be styled.
    ///
    pub fn new(config: RegexConfig) -> Result<Self, Error> {
        let regex = Regex::new(config.regex.as_str())?;

        Ok(Self {
            regex,
            style: config.style.into(),
        })
    }
}

impl Highlight for RegexpHighlighter {
    fn apply(&self, input: &str) -> String {
        let regex = &self.regex;
        let color = &self.style;
        let capture_groups = regex.captures_len() - 1;

        let mut new_string = String::new();
        let mut last_end = 0;

        for caps in regex.captures_iter(input) {
            if let Some(entire_match) = caps.get(0) {
                // Add the text before the regex match
                new_string.push_str(&get_pre_match_text(input, last_end, entire_match.start()));

                // Determine what part of the match to highlight
                match capture_groups {
                    1 => {
                        if let Some(captured) = caps.get(1) {
                            // Add the text before the capturing group (from the start of the entire match)
                            new_string.push_str(&get_pre_match_text(input, entire_match.start(), captured.start()));
                            // Highlight the captured group
                            new_string.push_str(&format!("{}", color.paint(captured.as_str())));
                            // Add the text after the capturing group (up to the end of the entire match)
                            new_string.push_str(&get_pre_match_text(input, captured.end(), entire_match.end()));
                        }
                    }
                    _ => {
                        // No capturing groups or more than one, highlight the entire match
                        let captured = entire_match.as_str();
                        new_string.push_str(&format!("{}", color.paint(captured)));
                    }
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
