use crate::core::config::NumberConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct NumberHighlighter {
    regex: Regex,
    style: NuStyle,
}

impl NumberHighlighter {
    pub fn new(config: NumberConfig) -> Result<Self, Error> {
        let pattern = r"(?x)
            \b          # start of number
            \d+         # integer part
            (?:\.\d+)?  # optional fractional
            \b          # end of number
        ";

        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            style: config.style.into(),
        })
    }
}

impl Highlight for NumberHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut it = self.regex.find_iter(input).peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(input);
        }

        let mut out = String::with_capacity(input.len() + 32);
        let mut last = 0usize;

        for m in self.regex.find_iter(input) {
            out.push_str(&input[last..m.start()]);
            let _ = write!(out, "{}", self.style.paint(&input[m.start()..m.end()]));
            last = m.end();
        }
        out.push_str(&input[last..]);

        Cow::Owned(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

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
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
