use crate::core::config::KeyValueConfig;
use crate::core::highlighter::Highlight;
use memchr::memchr;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct KeyValueHighlighter {
    regex: Regex,
    key: NuStyle,
    separator: NuStyle,
}

impl KeyValueHighlighter {
    pub fn new(config: KeyValueConfig) -> Result<Self, Error> {
        let pattern = r"(?P<space_or_start>(^)|\s)(?P<key>\w+\b)(?P<equals>=)";
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            key: config.key.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for KeyValueHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b'=', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all(input, |captures: &Captures| {
            let space_or_start = captures.name("space_or_start").map(|s| s.as_str()).unwrap_or_default();
            let mut buf = String::with_capacity(space_or_start.len() + 32);
            buf.push_str(space_or_start);
            if let Some(k) = captures.name("key") {
                let _ = write!(buf, "{}", self.key.paint(k.as_str()));
            }
            if let Some(e) = captures.name("equals") {
                let _ = write!(buf, "{}", self.separator.paint(e.as_str()));
            }
            buf
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_number_highlighter() {
        let highlighter = KeyValueHighlighter::new(KeyValueConfig {
            key: Style::new().fg(Color::Red),
            separator: Style::new().fg(Color::Yellow),
        })
        .unwrap();

        let cases = vec![
            ("Entry key=value", "Entry [red]key[reset][yellow]=[reset]value"),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
