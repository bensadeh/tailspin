use crate::core::config::KeywordConfig;
use crate::core::core::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct KeywordHighlighter {
    regex: Regex,
    style: NuStyle,
}

impl KeywordHighlighter {
    pub fn new(keyword_config: KeywordConfig) -> Result<Self, Error> {
        let keyword_pattern = keyword_config
            .words
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>()
            .join("|");

        let regex = Regex::new(&format!(r"\b({})\b", keyword_pattern))?;

        Ok(Self {
            regex,
            style: keyword_config.style.into(),
        })
    }
}

impl Highlight for KeywordHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| match self.style.background {
                None => {
                    format!("{}", self.style.paint(&caps[0]))
                }
                Some(_) => {
                    let capture_with_extra_padding = format!(" {} ", &caps[0]);
                    format!("{}", self.style.paint(capture_with_extra_padding))
                }
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::{Color, Style};

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
            assert_eq!(expected, actual.convert_escape_codes());
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
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
