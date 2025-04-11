use crate::core::config::DateTimeConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};
use std::borrow::Cow;

pub struct DateDashHighlighter {
    regex_yyyy_xx_xx: Regex,
    regex_xx_xx_yyyy: Regex,
    date: NuStyle,
    separator: NuStyle,
}

impl DateDashHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let regex_yyyy_xx_xx = Regex::new(
            r"(?x)
                (?P<year>19\d{2}|20\d{2})            # Year: 1900-2099
                (?P<separator>[-/])                  # Separator (dash or slash)
                (?P<first>0[1-9]|[12]\d|3[01])       # First number: 01-31
                (?P<separator2>[-/])                 # Separator (dash or slash)
                (?P<second>0[1-9]|[12]\d|3[01])      # Second number: 01-31
                ",
        )?;

        let regex_xx_xx_yyyy = Regex::new(
            r"(?x)
                (?P<first>0[1-9]|[12]\d|3[01])       # First number: 01-31
                (?P<separator>[-/])                  # Separator (dash or slash)
                (?P<second>0[1-9]|[12]\d|3[01])      # Second number: 01-31
                (?P<separator2>[-/])                 # Separator (dash or slash)
                (?P<year>19\d{2}|20\d{2})            # Year: 1900-2099
                ",
        )?;

        Ok(Self {
            regex_yyyy_xx_xx,
            regex_xx_xx_yyyy,
            date: time_config.date.into(),
            separator: time_config.separator.into(),
        })
    }

    fn highlight_date(&self, caps: &Captures<'_>) -> Option<String> {
        let year = caps.name("year").map(|m| self.date.paint(m.as_str()));
        let first = caps.name("first").map(|m| self.date.paint(m.as_str()));
        let second = caps.name("second").map(|m| self.date.paint(m.as_str()));
        let separator1 = caps.name("separator").map(|m| self.separator.paint(m.as_str()));
        let separator2 = caps.name("separator2").map(|m| self.separator.paint(m.as_str()));

        match (year, first, second, separator1, separator2) {
            (Some(y), Some(f), Some(s), Some(s1), Some(s2)) => Some(format!("{}{}{}{}{}", y, s1, f, s2, s)),
            _ => None,
        }
    }

    fn apply_regexes<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut changed = false;

        let res1 = self.regex_yyyy_xx_xx.replace_all(input, |caps: &Captures<'_>| {
            changed = true;
            self.highlight_date(caps).unwrap_or_else(|| caps[0].to_string())
        });

        let res2 = self.regex_xx_xx_yyyy.replace_all(res1.as_ref(), |caps: &Captures<'_>| {
            changed = true;
            self.highlight_date(caps).unwrap_or_else(|| caps[0].to_string())
        });

        if changed {
            Cow::Owned(res2.into_owned())
        } else {
            Cow::Borrowed(input)
        }
    }
}

impl Highlight for DateDashHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.apply_regexes(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_date_dash_highlighter() {
        let config = DateTimeConfig {
            date: Style::new().fg(Color::Magenta),
            separator: Style::new().fg(Color::Blue),
            ..DateTimeConfig::default()
        };
        let highlighter = DateDashHighlighter::new(config).unwrap();

        let cases = vec![
            (
                "2022-09-09",
                "[magenta]2022[reset][blue]-[reset][magenta]09[reset][blue]-[reset][magenta]09[reset]",
            ),
            (
                "2022/12/30",
                "[magenta]2022[reset][blue]/[reset][magenta]12[reset][blue]/[reset][magenta]30[reset]",
            ),
            (
                "09-09-2022",
                "[magenta]2022[reset][blue]-[reset][magenta]09[reset][blue]-[reset][magenta]09[reset]",
            ),
            (
                "09/09/2022",
                "[magenta]2022[reset][blue]/[reset][magenta]09[reset][blue]/[reset][magenta]09[reset]",
            ),
            ("3022-09-09", "3022-09-09"), // invalid year
            ("2022-19-39", "2022-19-39"), // invalid month
            ("2022/19/39", "2022/19/39"), // invalid month
            ("19/39/3023", "19/39/3023"), // invalid year
            ("No dates here!", "No dates here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
