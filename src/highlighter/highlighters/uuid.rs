use crate::highlighter::config::UuidConfig;
use crate::highlighter::core::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct UuidHighlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    dash: NuStyle,
}

impl UuidHighlighter {
    pub fn new(config: UuidConfig) -> Result<Self, Error> {
        const UUID_REGEX: &str = r"(?x)       # Enable comments and whitespace insensitivity
            \b[0-9a-fA-F]{8}\b    # Match first segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match second segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match third segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{4}\b    # Match fourth segment of UUID
            -                     # Match separator
            \b[0-9a-fA-F]{12}\b   # Match last segment of UUID
            ";

        let regex = Regex::new(UUID_REGEX)?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            dash: config.dash.into(),
        })
    }
}

impl Highlight for UuidHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
                caps[0]
                    .chars()
                    .map(|c| match c {
                        '0'..='9' => format!("{}", self.number.paint(c.to_string())),
                        'a'..='f' | 'A'..='F' => format!("{}", self.letter.paint(c.to_string())),
                        '-' => format!("{}", self.dash.paint(c.to_string())),
                        _ => c.to_string(),
                    })
                    .collect::<String>()
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlighter::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::{Color, Style};

    #[test]
    fn test_uuid_highlighter() {
        let highlighter = UuidHighlighter::new(UuidConfig {
            number: Style::new().fg(Color::Cyan),
            letter: Style::new().fg(Color::Yellow),
            dash: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "The UUID is 123e4567-e89b-12d3-a456-426614174000.",
                "The UUID is [cyan]1[reset][cyan]2[reset][cyan]3[reset][yellow]e[reset][cyan]4[reset][cyan]5[reset][cyan]6[reset][cyan]7[reset][red]-[reset][yellow]e[reset][cyan]8[reset][cyan]9[reset][yellow]b[reset][red]-[reset][cyan]1[reset][cyan]2[reset][yellow]d[reset][cyan]3[reset][red]-[reset][yellow]a[reset][cyan]4[reset][cyan]5[reset][cyan]6[reset][red]-[reset][cyan]4[reset][cyan]2[reset][cyan]6[reset][cyan]6[reset][cyan]1[reset][cyan]4[reset][cyan]1[reset][cyan]7[reset][cyan]4[reset][cyan]0[reset][cyan]0[reset][cyan]0[reset].",
            ),
            (
                "Another UUID is f47ac10b-58cc-4372-a567-0e02b2c3d479.",
                "Another UUID is [yellow]f[reset][cyan]4[reset][cyan]7[reset][yellow]a[reset][yellow]c[reset][cyan]1[reset][cyan]0[reset][yellow]b[reset][red]-[reset][cyan]5[reset][cyan]8[reset][yellow]c[reset][yellow]c[reset][red]-[reset][cyan]4[reset][cyan]3[reset][cyan]7[reset][cyan]2[reset][red]-[reset][yellow]a[reset][cyan]5[reset][cyan]6[reset][cyan]7[reset][red]-[reset][cyan]0[reset][yellow]e[reset][cyan]0[reset][cyan]2[reset][yellow]b[reset][cyan]2[reset][yellow]c[reset][cyan]3[reset][yellow]d[reset][cyan]4[reset][cyan]7[reset][cyan]9[reset].",
            ),
            ("No UUID here!", "No UUID here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
