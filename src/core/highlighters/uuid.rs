use crate::core::config::UuidConfig;
use crate::core::highlighter::Highlight;
use memchr::memchr;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct UuidHighlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    dash: NuStyle,
}

impl UuidHighlighter {
    pub fn new(config: UuidConfig) -> Result<Self, Error> {
        let pattern = r"(?x)       # Enable comments and whitespace insensitivity
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

        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            dash: config.dash.into(),
        })
    }
}

fn has_at_least_n_dashes(bytes: &[u8], n: usize) -> bool {
    let mut start = 0;
    for _ in 0..n {
        match memchr(b'-', &bytes[start..]) {
            Some(pos) => start += pos + 1,
            None => return false,
        }
    }
    true
}

impl Highlight for UuidHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if !has_at_least_n_dashes(input.as_bytes(), 4) {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all(input, |caps: &Captures<'_>| {
            let matched = &caps[0];
            let mut buf = String::with_capacity(matched.len() + 32);
            for (i, c) in matched.char_indices() {
                let s = &matched[i..i + c.len_utf8()];
                let style = match c {
                    '0'..='9' => &self.number,
                    'a'..='f' | 'A'..='F' => &self.letter,
                    '-' => &self.dash,
                    _ => {
                        buf.push(c);
                        continue;
                    }
                };
                let _ = write!(buf, "{}", style.paint(s));
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
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
