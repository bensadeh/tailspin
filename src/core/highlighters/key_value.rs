use super::RegexExt;
use crate::core::config::KeyValueConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use memchr::memchr;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;

pub struct KeyValueHighlighter {
    regex: Regex,
    key: Painter,
    separator: Painter,
}

impl KeyValueHighlighter {
    pub fn new(config: KeyValueConfig) -> Result<Self, Error> {
        let pattern = r"(?P<space_or_start>(^)|\s)(?P<key>\w+\b)(?P<equals>=)";
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            key: Painter::new(config.key.into()),
            separator: Painter::new(config.separator.into()),
        })
    }
}

impl Highlight for KeyValueHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b'=', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            let space_or_start = caps.name("space_or_start").map(|s| s.as_str()).unwrap_or_default();
            buf.push_str(space_or_start);
            if let Some(k) = caps.name("key") {
                self.key.paint(buf, k.as_str());
            }
            if let Some(e) = caps.name("equals") {
                self.separator.paint(buf, e.as_str());
            }
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
