use crate::core::config::IpV6Config;
use crate::core::highlighter::Highlight;
use memchr::memchr;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;
use std::net::Ipv6Addr;

pub struct IpV6Highlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    separator: NuStyle,
}

impl IpV6Highlighter {
    pub fn new(config: IpV6Config) -> Result<Self, Error> {
        let pattern = r#"([0-9a-fA-F:.]{3,})(?:(/)(\d{1,3}))?"#;
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for IpV6Highlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b':', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex
            .replace_all(input, |caps: &Captures<'_>| match caps[1].parse::<Ipv6Addr>() {
                Ok(_ip) => {
                    let addr = &caps[1];
                    let mut output = String::with_capacity(addr.len() + 32);
                    for (i, c) in addr.char_indices() {
                        let s = &addr[i..i + c.len_utf8()];
                        let style = match c {
                            '0'..='9' => &self.number,
                            'a'..='f' | 'A'..='F' => &self.letter,
                            ':' | '.' => &self.separator,
                            _ => {
                                output.push(c);
                                continue;
                            }
                        };
                        let _ = write!(output, "{}", style.paint(s));
                    }

                    if let (Some(slash), Some(netmask)) = (caps.get(2), caps.get(3)) {
                        let _ = write!(output, "{}", self.separator.paint(slash.as_str()));
                        let _ = write!(output, "{}", self.number.paint(netmask.as_str()));
                    }

                    output
                }
                Err(_) => caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_ip_v6_highlighter() {
        let highlighter = IpV6Highlighter::new(IpV6Config {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Yellow),
            separator: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "2001:db8:0:0:0:ff00:42:8329",
                "[blue]2[reset][blue]0[reset][blue]0[reset][blue]1[reset][red]:[reset][yellow]d[reset][yellow]b[reset][blue]8[reset][red]:[reset][blue]0[reset][red]:[reset][blue]0[reset][red]:[reset][blue]0[reset][red]:[reset][yellow]f[reset][yellow]f[reset][blue]0[reset][blue]0[reset][red]:[reset][blue]4[reset][blue]2[reset][red]:[reset][blue]8[reset][blue]3[reset][blue]2[reset][blue]9[reset]",
            ),
            (
                "2001:db8::ff00:42:8329",
                "[blue]2[reset][blue]0[reset][blue]0[reset][blue]1[reset][red]:[reset][yellow]d[reset][yellow]b[reset][blue]8[reset][red]:[reset][red]:[reset][yellow]f[reset][yellow]f[reset][blue]0[reset][blue]0[reset][red]:[reset][blue]4[reset][blue]2[reset][red]:[reset][blue]8[reset][blue]3[reset][blue]2[reset][blue]9[reset]",
            ),
            ("::1", "[red]:[reset][red]:[reset][blue]1[reset]"),
            (
                "2001:db8:85a3::8a2e:192.0.2.33",
                "[blue]2[reset][blue]0[reset][blue]0[reset][blue]1[reset][red]:[reset][yellow]d[reset][yellow]b[reset][blue]8[reset][red]:[reset][blue]8[reset][blue]5[reset][yellow]a[reset][blue]3[reset][red]:[reset][red]:[reset][blue]8[reset][yellow]a[reset][blue]2[reset][yellow]e[reset][red]:[reset][blue]1[reset][blue]9[reset][blue]2[reset][red].[reset][blue]0[reset][red].[reset][blue]2[reset][red].[reset][blue]3[reset][blue]3[reset]",
            ),
            (
                "::ffff:127.0.0.1",
                "[red]:[reset][red]:[reset][yellow]f[reset][yellow]f[reset][yellow]f[reset][yellow]f[reset][red]:[reset][blue]1[reset][blue]2[reset][blue]7[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset][red].[reset][blue]1[reset]",
            ),
            (
                "fe80::/10",
                "[yellow]f[reset][yellow]e[reset][blue]8[reset][blue]0[reset][red]:[reset][red]:[reset][red]/[reset][blue]10[reset]",
            ),
            ("Not ipv4: 192.168.0.1", "Not ipv4: 192.168.0.1"),
            ("11:47:39:850", "11:47:39:850"),
            ("123/234/345/456", "123/234/345/456"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
