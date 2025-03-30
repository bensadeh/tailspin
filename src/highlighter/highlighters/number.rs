use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

use crate::highlighter::core::Highlight;
use crate::NumberConfig;

pub struct NumberHighlighter {
    regex: Regex,
    style: NuStyle,
}

impl NumberHighlighter {
    pub fn new(config: NumberConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)             # Enable verbose mode to allow comments and ignore whitespace
            \b                 # Match a word boundary (start of the number)
            \d+                # Match one or more digits (integer part of the number)
            (\.\d+)?           # Optionally match a dot followed by one or more digits (fractional part of the number)
            \b                 # Match a word boundary (end of the number)
            ",
        )?;

        Ok(Self {
            regex,
            style: config.style.into(),
        })
    }
}

impl Highlight for NumberHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| format!("{}", self.style.paint(&caps[0])))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlighter::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::{Color, Style};

    #[test]
    fn test_number_highlighter() {
        let highlighter = NumberHighlighter::new(NumberConfig {
            style: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "The fox jumps over 13 dogs. The number 42.5 is here.",
                "The fox jumps over [red]13[reset] dogs. The number [red]42.5[reset] is here.",
            ),
            (
                "There are 1001 nights in the tale.",
                "There are [red]1001[reset] nights in the tale.",
            ),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
