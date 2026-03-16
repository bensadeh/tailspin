use super::RegexExt;
use crate::core::config::PointerConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use memchr::memchr2;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;

pub struct PointerHighlighter {
    regex: Regex,
    number: Painter,
    letter: Painter,
    separator: Painter,
    separator_token_str: String,
    x: Painter,
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
            number: Painter::new(config.number.into()),
            letter: Painter::new(config.letter.into()),
            separator: Painter::new(config.separator.into()),
            separator_token_str: config.separator_token.to_string(),
            x: Painter::new(config.x.into()),
        })
    }
}

impl PointerHighlighter {
    fn write_hex_chars(&self, buf: &mut String, text: &str) {
        for (i, c) in text.char_indices() {
            let s = &text[i..i + c.len_utf8()];
            let painter = match c {
                '0'..='9' => &self.number,
                'x' | 'X' => &self.x,
                'a'..='f' | 'A'..='F' => &self.letter,
                _ => {
                    buf.push(c);
                    continue;
                }
            };
            painter.paint(buf, s);
        }
    }
}

impl Highlight for PointerHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr2(b'x', b'X', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            let prefix = caps.name("prefix").or_else(|| caps.name("prefix64")).unwrap().as_str();
            let first_half = caps
                .name("first_half")
                .or_else(|| caps.name("first_half64"))
                .unwrap()
                .as_str();

            self.write_hex_chars(buf, prefix);
            self.write_hex_chars(buf, first_half);

            if let Some(second_half) = caps.name("second_half") {
                self.separator.paint(buf, &self.separator_token_str);
                self.write_hex_chars(buf, second_half.as_str());
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
