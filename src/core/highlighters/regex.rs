use crate::core::config::RegexConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use regex::{Error, Regex};
use std::borrow::Cow;

pub struct RegexpHighlighter {
    regex: Regex,
    style: Painter,
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
            style: Painter::new(config.style.into()),
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
                            self.style.paint(&mut new_string, captured.as_str());
                            new_string.push_str(&input[captured.end()..entire_match.end()]);
                        } else {
                            self.style.paint(&mut new_string, entire_match.as_str());
                        }
                    }
                    _ => {
                        self.style.paint(&mut new_string, entire_match.as_str());
                    }
                }
                last_end = entire_match.end();
            }
        }
        new_string.push_str(&input[last_end..]);

        Cow::Owned(new_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    fn make_highlighter(pattern: &str) -> RegexpHighlighter {
        let config = RegexConfig {
            regex: pattern.to_string(),
            style: Style::new().fg(Color::Red),
        };
        RegexpHighlighter::new(config).unwrap()
    }

    #[test]
    fn test_optional_capture_group_not_participating() {
        // Pattern with one optional capture group: group 1 participates
        // only when "error" is present
        let highlighter = make_highlighter(r"(error)?warning");

        // When the capture group does NOT participate, the matched text
        // ("warning") must still appear in the output.
        let result = highlighter.apply("got a warning here");
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(readable, "got a [red]warning[reset] here");

        // When the capture group DOES participate, only it is styled.
        let result = highlighter.apply("got a errorwarning here");
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(readable, "got a [red]error[reset]warning here");
    }
}
