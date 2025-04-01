use crate::core::config::RegexConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex};
use std::borrow::Cow;

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
        Ok(Self {
            regex: Regex::new(config.regex.as_str())?,
            style: config.style.into(),
        })
    }
}

impl Highlight for RegexpHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut new_string = String::new();
        let mut last_end = 0;
        let mut changed = false;
        let capture_groups = self.regex.captures_len() - 1;

        for caps in self.regex.captures_iter(input) {
            if let Some(entire_match) = caps.get(0) {
                changed = true;
                // Append text before the match (this is a slice from the original input)
                new_string.push_str(&input[last_end..entire_match.start()]);

                match capture_groups {
                    1 => {
                        if let Some(captured) = caps.get(1) {
                            // Append text from the start of the match until the capture group.
                            new_string.push_str(&input[entire_match.start()..captured.start()]);
                            // Append the highlighted capture group.
                            new_string.push_str(&format!("{}", self.style.paint(captured.as_str())));
                            // Append text from after the capture group until the end of the match.
                            new_string.push_str(&input[captured.end()..entire_match.end()]);
                        }
                    }
                    _ => {
                        // Highlight the entire match.
                        new_string.push_str(&format!("{}", self.style.paint(entire_match.as_str())));
                    }
                }
                last_end = entire_match.end();
            }
        }
        // Append any remaining text after the last match.
        new_string.push_str(&input[last_end..]);

        if changed {
            Cow::Owned(new_string)
        } else {
            Cow::Borrowed(input)
        }
    }
}
