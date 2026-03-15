use crate::core::config::RegexConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex};
use std::borrow::Cow;
use std::fmt::Write as _;

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
        let capture_groups = self.regex.captures_len() - 1;
        let mut caps_iter = self.regex.captures_iter(input).peekable();

        if caps_iter.peek().is_none() {
            return Cow::Borrowed(input);
        }

        let mut new_string = String::with_capacity(input.len() + 32);
        let mut last_end = 0;

        for caps in caps_iter {
            if let Some(entire_match) = caps.get(0) {
                new_string.push_str(&input[last_end..entire_match.start()]);

                match capture_groups {
                    1 => {
                        if let Some(captured) = caps.get(1) {
                            new_string.push_str(&input[entire_match.start()..captured.start()]);
                            let _ = write!(new_string, "{}", self.style.paint(captured.as_str()));
                            new_string.push_str(&input[captured.end()..entire_match.end()]);
                        }
                    }
                    _ => {
                        let _ = write!(new_string, "{}", self.style.paint(entire_match.as_str()));
                    }
                }
                last_end = entire_match.end();
            }
        }
        new_string.push_str(&input[last_end..]);

        Cow::Owned(new_string)
    }
}
