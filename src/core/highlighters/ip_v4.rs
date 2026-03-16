use super::RegexExt;
use crate::core::config::IpV4Config;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use memchr::memchr;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;

pub struct IpV4Highlighter {
    regex: Regex,
    segment: Painter,
    separator: Painter,
}

impl IpV4Highlighter {
    pub fn new(config: IpV4Config) -> Result<Self, Error> {
        let pattern = r"(?x)\b
            (?P<o1>\d{1,3})\.
            (?P<o2>\d{1,3})\.
            (?P<o3>\d{1,3})\.
            (?P<o4>\d{1,3})
            (?:/(?P<mask>\d{1,2}))?
            \b";
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            segment: Painter::new(config.number.into()),
            separator: Painter::new(config.separator.into()),
        })
    }
}

impl Highlight for IpV4Highlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b'.', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        let names = ["o1", "o2", "o3", "o4"];

        self.regex.replace_all_cow(input, |caps, buf| {
            let valid_octets = names
                .iter()
                .all(|n| caps.name(n).unwrap().as_str().parse::<u8>().is_ok());
            let valid_mask = caps
                .name("mask")
                .map_or(true, |ms| ms.as_str().parse::<u8>().is_ok_and(|v| v <= 32));

            if valid_octets && valid_mask {
                for (i, &n) in names.iter().enumerate() {
                    self.segment.paint(buf, caps.name(n).unwrap().as_str());
                    if i < 3 {
                        self.separator.paint(buf, ".");
                    }
                }
                if let Some(ms) = caps.name("mask") {
                    self.separator.paint(buf, "/");
                    self.segment.paint(buf, ms.as_str());
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
    fn test_ipv4_highlighter_valid() {
        let h = IpV4Highlighter::new(IpV4Config {
            number: Style::new().fg(Color::Blue),
            separator: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "10.0.0.123",
                "[blue]10[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset][red].[reset][blue]123[reset]",
            ),
            (
                "192.168.0.1/24",
                "[blue]192[reset][red].[reset][blue]168[reset][red].[reset][blue]0[reset][red].[reset][blue]1[reset][red]/[reset][blue]24[reset]",
            ),
            (
                "0.0.0.0",
                "[blue]0[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset]",
            ),
        ];

        for (input, expect) in cases {
            let actual = h.apply(input);
            assert_eq!(expect, actual.to_string().convert_escape_codes());
        }
    }

    #[test]
    fn test_ipv4_highlighter_invalid_octet_or_mask() {
        let h = IpV4Highlighter::new(IpV4Config {
            number: Style::new().fg(Color::Blue),
            separator: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            // octet >255
            ("256.1.1.1", "256.1.1.1"),
            // octet >255
            ("999.999.999.999", "999.999.999.999"),
            // mask >32
            ("192.168.0.1/33", "192.168.0.1/33"),
            // partial (too few segments) shouldn’t match at all
            ("1.2.3", "1.2.3"),
        ];

        for (input, expect) in cases {
            let actual = h.apply(input);
            assert_eq!(expect, actual);
        }
    }
}
