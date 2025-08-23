use crate::core::config::KeywordConfig;
use crate::core::highlighter::Highlight;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use nu_ansi_term::Style as NuStyle;
use regex::Error;
use std::borrow::Cow;

pub struct KeywordHighlighter {
    ac: AhoCorasick,
    style: NuStyle,
}

impl KeywordHighlighter {
    pub fn new(keyword_config: KeywordConfig) -> Result<Self, Error> {
        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .byte_classes(true)
            .build(&keyword_config.words)
            .map_err(|e| Error::Syntax(e.to_string()))?;

        Ok(Self {
            ac,
            style: keyword_config.style.into(),
        })
    }
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_uppercase() || b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'
}

fn is_word_boundary(hay: &[u8], start: usize, end: usize) -> bool {
    let left_ok = start == 0 || !is_word_byte(hay[start - 1]);
    let right_ok = end == hay.len() || !is_word_byte(hay[end]);
    left_ok && right_ok
}

impl Highlight for KeywordHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let bytes = input.as_bytes();
        let mut out: Option<String> = None;
        let mut last = 0;

        for m in self.ac.find_iter(bytes) {
            let (s, e) = (m.start(), m.end());
            if !is_word_boundary(bytes, s, e) {
                continue;
            }

            let out_buf = out.get_or_insert_with(|| String::with_capacity(input.len() + 16));

            out_buf.push_str(&input[last..s]);

            if self.style.background.is_none() {
                out_buf.push_str(&format!("{}", self.style.paint(&input[s..e])));
            } else {
                let padded = format!(" {} ", &input[s..e]);
                out_buf.push_str(&format!("{}", self.style.paint(padded)));
            }

            last = e;
        }

        match out {
            None => Cow::Borrowed(input),
            Some(mut s) => {
                s.push_str(&input[last..]);
                Cow::Owned(s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_foreground_keyword_highlighter() {
        let config = KeywordConfig {
            words: vec!["null".to_string()],
            style: Style::new().fg(Color::Red),
        };
        let highlighter = KeywordHighlighter::new(config).unwrap();

        let cases = vec![
            ("Hello null world", "Hello [red]null[reset] world"),
            (
                "There are 1001 nights in the tale.",
                "There are 1001 nights in the tale.",
            ),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }

    #[test]
    fn test_background_keyword_highlighter() {
        let config = KeywordConfig {
            words: vec!["null".to_string()],
            style: Style::new().on(Color::Red),
        };
        let highlighter = KeywordHighlighter::new(config).unwrap();

        let cases = vec![
            ("Hello null world", "Hello [bg_red] null [reset] world"),
            (
                "There are 1001 nights in the tale.",
                "There are 1001 nights in the tale.",
            ),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
