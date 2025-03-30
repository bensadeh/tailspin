use crate::core::config::KeyValueConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct KeyValueHighlighter {
    regex: Regex,
    key: NuStyle,
    separator: NuStyle,
}

impl KeyValueHighlighter {
    pub fn new(config: KeyValueConfig) -> Result<Self, Error> {
        let regex = Regex::new(r"(?P<space_or_start>(^)|\s)(?P<key>\w+\b)(?P<equals>=)")?;

        Ok(Self {
            regex,
            key: config.key.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for KeyValueHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |captures: &Captures| {
                let space_or_start = captures.name("space_or_start").map(|s| s.as_str()).unwrap_or_default();
                let key = captures
                    .name("key")
                    .map(|k| format!("{}", self.key.paint(k.as_str())))
                    .unwrap_or_default();
                let equals_sign = captures
                    .name("equals")
                    .map(|e| format!("{}", self.separator.paint(e.as_str())))
                    .unwrap_or_default();

                format!("{}{}{}", space_or_start, key, equals_sign)
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
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
