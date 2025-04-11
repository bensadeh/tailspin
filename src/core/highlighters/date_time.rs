use crate::core::config::DateTimeConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex};
use std::borrow::Cow;

pub struct TimeHighlighter {
    regex: Regex,
    time: NuStyle,
    zone: NuStyle,
    separator: NuStyle,
}

impl TimeHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)
            (?P<T>[T\s])?                              
            (?P<hours>[01]?\d|2[0-3])(?P<colon1>:)
            (?P<minutes>[0-5]\d)(?P<colon2>:)
            (?P<seconds>[0-5]\d)
            (?P<frac_sep>[.,:])?(?P<frac_digits>\d+)?  
            (?P<tz>Z)?            
            ",
        )?;

        Ok(Self {
            regex,
            time: time_config.time.into(),
            zone: time_config.zone.into(),
            separator: time_config.separator.into(),
        })
    }
}

impl Highlight for TimeHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.regex.replace_all(input, |caps: &regex::Captures<'_>| {
            let paint_and_stringify = |name: &str, style: &NuStyle| {
                caps.name(name)
                    .map(|m| style.paint(m.as_str()).to_string())
                    .unwrap_or_default()
            };

            let parts = [
                ("T", &self.zone),
                ("hours", &self.time),
                ("colon1", &self.separator),
                ("minutes", &self.time),
                ("colon2", &self.separator),
                ("seconds", &self.time),
                ("frac_sep", &self.separator),
                ("frac_digits", &self.time),
                ("tz", &self.zone),
            ];

            parts.iter().fold(String::new(), |acc, (name, style)| {
                acc + &paint_and_stringify(name, style)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::DateTimeConfig;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_time_highlighter() {
        let config = DateTimeConfig {
            date: Style::new(),
            time: Style::new().fg(Color::Red),
            zone: Style::new().fg(Color::Blue),
            separator: Style::new().fg(Color::Yellow),
        };
        let highlighter = TimeHighlighter::new(config).unwrap();

        let cases = vec![
            (
                "07:46:34",
                "[red]07[reset][yellow]:[reset][red]46[reset][yellow]:[reset][red]34[reset]",
            ),
            (
                "10:51:19.251",
                "[red]10[reset][yellow]:[reset][red]51[reset][yellow]:[reset][red]19[reset][yellow].[reset][red]251[reset]",
            ),
            (
                "11:47:39:850",
                "[red]11[reset][yellow]:[reset][red]47[reset][yellow]:[reset][red]39[reset][yellow]:[reset][red]850[reset]",
            ),
            (
                "3:33:30",
                "[red]3[reset][yellow]:[reset][red]33[reset][yellow]:[reset][red]30[reset]",
            ),
            (
                "2022-09-09 11:48:34,534",
                "2022-09-09[blue] [reset][red]11[reset][yellow]:[reset][red]48[reset][yellow]:[reset][red]34[reset][yellow],[reset][red]534[reset]",
            ),
            (
                "2022-09-22T07:46:34.171800155Z",
                "2022-09-22[blue]T[reset][red]07[reset][yellow]:[reset][red]46[reset][yellow]:[reset][red]34[reset][yellow].[reset][red]171800155[reset][blue]Z[reset]",
            ),
            (
                "2024-09-14T07:57:30.659+02:00",
                "2024-09-14[blue]T[reset][red]07[reset][yellow]:[reset][red]57[reset][yellow]:[reset][red]30[reset][yellow].[reset][red]659[reset]+02:00",
            ),
            ("No time here!", "No time here!"),
            ("2001:db8::ff00:42:8329", "2001:db8::ff00:42:8329"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
