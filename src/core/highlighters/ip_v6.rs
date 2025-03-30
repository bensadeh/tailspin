use crate::core::config::IpV6Config;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};
use std::net::Ipv6Addr;

pub struct IpV6Highlighter {
    regex: Regex,
    number: NuStyle,
    letter: NuStyle,
    separator: NuStyle,
}

impl IpV6Highlighter {
    pub fn new(config: IpV6Config) -> Result<Self, Error> {
        let regex = Regex::new(r#"([0-9a-fA-F\:\.]{3,})(?:(/)(\d{1,3}))?"#)?;

        Ok(Self {
            regex,
            number: config.number.into(),
            letter: config.letter.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for IpV6Highlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| match caps[1].parse::<Ipv6Addr>() {
                Ok(_ip) => {
                    let mut output = caps[1]
                        .chars()
                        .map(|c| match c {
                            '0'..='9' => self.number.paint(c.to_string()).to_string(),
                            'a'..='f' | 'A'..='F' => self.letter.paint(c.to_string()).to_string(),
                            ':' | '.' => self.separator.paint(c.to_string()).to_string(),
                            _ => c.to_string(),
                        })
                        .collect::<String>();

                    let slash = caps.get(2);
                    let netmask = caps.get(3);
                    if let (Some(slash), Some(netmask)) = (slash, netmask) {
                        output.push_str(&self.separator.paint(slash.as_str()).to_string());
                        output.push_str(&self.number.paint(netmask.as_str()).to_string());
                    }

                    output
                }
                Err(_err) => caps[1].to_string(),
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
                "::ffff:127.0.0.1",
                "[red]:[reset][red]:[reset][yellow]f[reset][yellow]f[reset][yellow]f[reset][yellow]f[reset][red]:[reset][blue]1[reset][blue]2[reset][blue]7[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset][red].[reset][blue]1[reset]",
            ),
            (
                "fe80::/10",
                "[yellow]f[reset][yellow]e[reset][blue]8[reset][blue]0[reset][red]:[reset][red]:[reset][red]/[reset][blue]10[reset]",
            ),
            ("Not ipv4: 192.168.0.1", "Not ipv4: 192.168.0.1"),
            ("11:47:39:850", "11:47:39:850"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
