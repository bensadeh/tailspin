use super::RegexExt;
use crate::core::config::IpV6Config;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use memchr::memchr;
use regex::{Regex, RegexBuilder};
use std::borrow::Cow;
use std::net::Ipv6Addr;

pub struct IpV6Highlighter {
    regex: Regex,
    number: Painter,
    letter: Painter,
    separator: Painter,
}

impl IpV6Highlighter {
    pub fn new(config: IpV6Config) -> Self {
        let pattern = r"([0-9a-fA-F:.]{3,})(?:(/)(\d{1,3}))?";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded IPv6 regex must compile");

        Self {
            regex,
            number: Painter::new(config.number.into()),
            letter: Painter::new(config.letter.into()),
            separator: Painter::new(config.separator.into()),
        }
    }
}

impl Highlight for IpV6Highlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b':', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            if caps[1].parse::<Ipv6Addr>().is_ok() {
                let addr = &caps[1];
                for (i, c) in addr.char_indices() {
                    let s = &addr[i..i + c.len_utf8()];
                    let painter = match c {
                        '0'..='9' => &self.number,
                        'a'..='f' | 'A'..='F' => &self.letter,
                        ':' | '.' => &self.separator,
                        _ => {
                            buf.push(c);
                            continue;
                        }
                    };
                    painter.paint(buf, s);
                }

                if let (Some(slash), Some(netmask)) = (caps.get(2), caps.get(3)) {
                    self.separator.paint(buf, slash.as_str());
                    self.number.paint(buf, netmask.as_str());
                }
            } else {
                buf.push_str(caps.get(0).unwrap().as_str());
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
    fn test_ip_v6_highlighter() {
        let highlighter = IpV6Highlighter::new(IpV6Config {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Yellow),
            separator: Style::new().fg(Color::Red),
        });

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
