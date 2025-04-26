use crate::core::config::IpV4Config;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};
use std::borrow::Cow;

pub struct IpV4Highlighter {
    regex: Regex,
    segment_style: NuStyle,
    separator_style: NuStyle,
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

        Ok(Self {
            regex: Regex::new(pattern)?,
            segment_style: config.number.into(),
            separator_style: config.separator.into(),
        })
    }
}

impl Highlight for IpV4Highlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let seg = &self.segment_style;
        let sep = &self.separator_style;

        self.regex
            .replace_all(input, |caps: &Captures<'_>| highlight_caps(seg, sep, caps))
    }
}

fn highlight_caps(seg_style: &NuStyle, sep_style: &NuStyle, caps: &Captures<'_>) -> String {
    let full_match = caps.get(0).expect("full match always present").as_str();

    let names = ["o1", "o2", "o3", "o4"];
    for &n in &names {
        let txt = caps.name(n).expect("named octet group always present").as_str();
        if txt.parse::<u8>().is_err() {
            return full_match.to_string();
        }
    }

    let mask_str = caps.name("mask").map(|m| m.as_str());
    if let Some(ms) = mask_str {
        if !ms.parse::<u8>().map(|v| v <= 32).unwrap_or(false) {
            return full_match.to_string();
        }
    }

    let mut output = String::new();
    for (i, &n) in names.iter().enumerate() {
        let text = caps.name(n).expect("named octet group always present").as_str();
        output.push_str(&seg_style.paint(text).to_string());
        if i < 3 {
            output.push_str(&sep_style.paint(".").to_string());
        }
    }

    if let Some(ms) = mask_str {
        output.push_str(&sep_style.paint("/").to_string());
        output.push_str(&seg_style.paint(ms).to_string());
    }

    output
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
