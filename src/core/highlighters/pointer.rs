use crate::core::config::PointerConfig;
use crate::core::highlighter::Highlight;
use memchr::memchr2;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

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
        let pattern = r"(?ix)
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
        ";

        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

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

impl PointerHighlighter {
    fn write_hex_chars(&self, buf: &mut String, text: &str) {
        for (i, c) in text.char_indices() {
            let s = &text[i..i + c.len_utf8()];
            let style = match c {
                '0'..='9' => &self.number,
                'x' | 'X' => &self.x,
                'a'..='f' | 'A'..='F' => &self.letter,
                _ => {
                    buf.push(c);
                    continue;
                }
            };
            let _ = write!(buf, "{}", style.paint(s));
        }
    }
}

impl Highlight for PointerHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr2(b'x', b'X', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all(input, |caps: &Captures<'_>| {
            let prefix = caps.name("prefix").or_else(|| caps.name("prefix64")).unwrap().as_str();
            let first_half = caps
                .name("first_half")
                .or_else(|| caps.name("first_half64"))
                .unwrap()
                .as_str();

            let mut buf = String::with_capacity(prefix.len() + first_half.len() * 2 + 32);
            self.write_hex_chars(&mut buf, prefix);
            self.write_hex_chars(&mut buf, first_half);

            if let Some(second_half) = caps.name("second_half") {
                let sep: &str = &self.separator_token.to_string();
                let _ = write!(buf, "{}", self.separator.paint(sep));
                self.write_hex_chars(&mut buf, second_half.as_str());
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
