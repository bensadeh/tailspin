use crate::core::config::PointerConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};
use std::borrow::Cow;

pub struct PointerHighlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    separator: NuStyle,
    separator_token: char,
    x: NuStyle,
}

impl PointerHighlighter {
    pub fn new(config: PointerConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?ix)
            \b
            (?P<prefix>0x)
            (?P<first_half>[0-9a-fA-F]{8})
            \b          
            |
            \b
            (?P<prefix64>0x)
            (?P<first_half64>[0-9a-fA-F]{8})
            (?P<second_half>[0-9a-fA-F]{8})
            \b  
        ",
        )?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            separator: config.separator.into(),
            separator_token: config.separator_token,
            x: config.x.into(),
        })
    }
}

impl Highlight for PointerHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.regex.replace_all(input, |caps: &Captures<'_>| {
            let prefix = caps.name("prefix").or_else(|| caps.name("prefix64")).unwrap().as_str();
            let first_half = caps
                .name("first_half")
                .or_else(|| caps.name("first_half64"))
                .unwrap()
                .as_str();
            let formatted_prefix = prefix
                .chars()
                .map(|c| highlight_char(c, self.number, self.x, self.letter))
                .collect::<String>();
            let formatted_first_half = first_half
                .chars()
                .map(|c| highlight_char(c, self.number, self.x, self.letter))
                .collect::<String>();

            caps.name("second_half").map_or_else(
                || format!("{}{}", formatted_prefix, formatted_first_half),
                |second_half| {
                    let formatted_second_half = second_half
                        .as_str()
                        .chars()
                        .map(|c| highlight_char(c, self.number, self.x, self.letter))
                        .collect::<String>();
                    format!(
                        "{}{}{}{}",
                        formatted_prefix,
                        formatted_first_half,
                        self.separator.paint(self.separator_token.to_string()),
                        formatted_second_half
                    )
                },
            )
        })
    }
}

fn highlight_char(c: char, number: NuStyle, x: NuStyle, letter: NuStyle) -> String {
    match c {
        '0'..='9' => format!("{}", number.paint(c.to_string())),
        'x' | 'X' => format!("{}", x.paint(c.to_string())),
        'a'..='f' | 'A'..='F' => format!("{}", letter.paint(c.to_string())),
        _ => c.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::{Color, Style};

    #[test]
    fn test_pointer_highlighter() {
        let highlighter = PointerHighlighter::new(PointerConfig {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Magenta),
            separator: Style::new().fg(Color::Green),
            separator_token: '•',
            x: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "0x8c2a0aeb",
                "[blue]0[reset][red]x[reset][blue]8[reset][magenta]c[reset][blue]2[reset][magenta]a[reset][blue]0[reset][magenta]a[reset][magenta]e[reset][magenta]b[reset]",
            ),
            (
                "0xd7b3b2f446e2c21b",
                "[blue]0[reset][red]x[reset][magenta]d[reset][blue]7[reset][magenta]b[reset][blue]3[reset][magenta]b[reset][blue]2[reset][magenta]f[reset][blue]4[reset][green]•[reset][blue]4[reset][blue]6[reset][magenta]e[reset][blue]2[reset][magenta]c[reset][blue]2[reset][blue]1[reset][magenta]b[reset]",
            ),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
