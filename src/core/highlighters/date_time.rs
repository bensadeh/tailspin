use crate::core::config::DateTimeConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct TimeHighlighter {
    regex: Regex,
    idx: Idx,
    time: NuStyle,
    zone: NuStyle,
    separator: NuStyle,
}

#[derive(Copy, Clone)]
struct Idx {
    t: usize,
    hours: usize,
    colon1: usize,
    minutes: usize,
    colon2: usize,
    seconds: usize,
    frac_sep: usize,
    frac_digits: usize,
    tz: usize,
}

impl TimeHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let pattern = r"(?x)
            (?P<T>[T\s])?
            (?P<hours>[01]?\d|2[0-3])(?P<colon1>:)
            (?P<minutes>[0-5]\d)(?P<colon2>:)
            (?P<seconds>[0-5]\d)
            (?P<frac_sep>[.,:])?(?P<frac_digits>\d+)?
            (?P<tz>Z)?
        ";

        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        let mut name_to_idx = std::collections::HashMap::new();
        for (i, name_opt) in regex.capture_names().enumerate() {
            if let Some(name) = name_opt {
                name_to_idx.insert(name.to_string(), i);
            }
        }
        let idx = Idx {
            t: name_to_idx["T"],
            hours: name_to_idx["hours"],
            colon1: name_to_idx["colon1"],
            minutes: name_to_idx["minutes"],
            colon2: name_to_idx["colon2"],
            seconds: name_to_idx["seconds"],
            frac_sep: name_to_idx["frac_sep"],
            frac_digits: name_to_idx["frac_digits"],
            tz: name_to_idx["tz"],
        };

        Ok(Self {
            regex,
            idx,
            time: time_config.time.into(),
            zone: time_config.zone.into(),
            separator: time_config.separator.into(),
        })
    }

    #[inline]
    fn write_part(buf: &mut String, caps: &Captures<'_>, i: usize, style: &NuStyle) {
        if let Some(m) = caps.get(i) {
            let _ = write!(buf, "{}", style.paint(m.as_str()));
        }
    }
}

impl Highlight for TimeHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut it = self.regex.captures_iter(input).peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(input);
        }

        let mut out = String::with_capacity(input.len() + 32);
        let mut last = 0usize;

        for caps in self.regex.captures_iter(input) {
            let m = caps.get(0).unwrap();
            out.push_str(&input[last..m.start()]);

            Self::write_part(&mut out, &caps, self.idx.t, &self.zone);
            Self::write_part(&mut out, &caps, self.idx.hours, &self.time);
            Self::write_part(&mut out, &caps, self.idx.colon1, &self.separator);
            Self::write_part(&mut out, &caps, self.idx.minutes, &self.time);
            Self::write_part(&mut out, &caps, self.idx.colon2, &self.separator);
            Self::write_part(&mut out, &caps, self.idx.seconds, &self.time);
            Self::write_part(&mut out, &caps, self.idx.frac_sep, &self.separator);
            Self::write_part(&mut out, &caps, self.idx.frac_digits, &self.time);
            Self::write_part(&mut out, &caps, self.idx.tz, &self.zone);

            last = m.end();
        }

        out.push_str(&input[last..]);
        Cow::Owned(out)
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
