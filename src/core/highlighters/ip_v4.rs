use crate::core::config::IpV4Config;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};
use std::borrow::Cow;

pub struct IpV4Highlighter {
    regex: Regex,
    number: NuStyle,
    separator: NuStyle,
}

impl IpV4Highlighter {
    pub fn new(config: IpV4Config) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)               # Enable verbose mode to allow comments and ignore whitespace
            (\b\d{1,3})          # Match 1 to 3 digits at a word boundary (start of the IP segment)
            (\.)                 # Match a literal dot (.)
            (\d{1,3})            # Match 1 to 3 digits (next IP segment)
            (\.)                 # Match a literal dot (.)
            (\d{1,3})            # Match 1 to 3 digits (next IP segment)
            (\.)                 # Match a literal dot (.)
            (\d{1,3})            # Match 1 to 3 digits at a word boundary
            (?:(/)(\d{1,2}))?    # Match optional netmask
            \b                   # End of the IP segment
    ",
        )?;

        Ok(Self {
            regex,
            number: config.number.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for IpV4Highlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let segment = &self.number;
        let separator = &self.separator;
        let highlight_groups = [
            segment, separator, segment, separator, segment, separator, segment, separator, segment,
        ];

        self.regex.replace_all(input, |caps: &Captures<'_>| {
            let mut output = String::new();
            for (group, cap) in caps.iter().enumerate().skip(1) {
                if let Some(cap) = cap {
                    let color = highlight_groups[group - 1];
                    output.push_str(&format!("{}", color.paint(cap.as_str())));
                }
            }
            output
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_ip_v4_highlighter() {
        let highlighter = IpV4Highlighter::new(IpV4Config {
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
                "192.168.0.1",
                "[blue]192[reset][red].[reset][blue]168[reset][red].[reset][blue]0[reset][red].[reset][blue]1[reset]",
            ),
            (
                "192.168.0.0/24",
                "[blue]192[reset][red].[reset][blue]168[reset][red].[reset][blue]0[reset][red].[reset][blue]0[reset][red]/[reset][blue]24[reset]",
            ),
            ("Invalid regex: 192.168.0", "Invalid regex: 192.168.0"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
