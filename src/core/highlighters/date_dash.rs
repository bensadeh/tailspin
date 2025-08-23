use crate::core::config::DateTimeConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct DateDashHighlighter {
    regex: Regex,
    date: NuStyle,
    separator: NuStyle,
    idx: Idx,
}

#[derive(Copy, Clone)]
struct Idx {
    // Branch A: YYYY sep first sep2 second
    a_year: usize,
    a_sep1: usize,
    a_first: usize,
    a_sep2: usize,
    a_second: usize,

    // Branch B: first sep second sep2 year
    b_first: usize,
    b_sep1: usize,
    b_second: usize,
    b_sep2: usize,
    b_year: usize,
}

impl DateDashHighlighter {
    pub fn new(time_config: DateTimeConfig) -> Result<Self, Error> {
        let pattern = r"(?x)
            # Branch A: YYYY-xx-xx
            (?P<a_year> 19\d{2} | 20\d{2} )
            (?P<a_sep1> [-/] )
            (?P<a_first> 0[1-9] | [12]\d | 3[01] )
            (?P<a_sep2> [-/] )
            (?P<a_second> 0[1-9] | [12]\d | 3[01] )
            |
            # Branch B: xx-xx-YYYY
            (?P<b_first> 0[1-9] | [12]\d | 3[01] )
            (?P<b_sep1>  [-/] )
            (?P<b_second> 0[1-9] | [12]\d | 3[01] )
            (?P<b_sep2>  [-/] )
            (?P<b_year>  19\d{2} | 20\d{2} )
        ";

        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        // Resolve capture names → indices once.
        let mut map = std::collections::HashMap::new();
        for (i, name) in regex.capture_names().enumerate() {
            if let Some(n) = name {
                map.insert(n.to_string(), i);
            }
        }
        let idx = Idx {
            a_year: map["a_year"],
            a_sep1: map["a_sep1"],
            a_first: map["a_first"],
            a_sep2: map["a_sep2"],
            a_second: map["a_second"],
            b_first: map["b_first"],
            b_sep1: map["b_sep1"],
            b_second: map["b_second"],
            b_sep2: map["b_sep2"],
            b_year: map["b_year"],
        };

        Ok(Self {
            regex,
            idx,
            date: time_config.date.into(),
            separator: time_config.separator.into(),
        })
    }

    #[inline]
    fn paint<'a>(&self, s: &'a str, style: &NuStyle, out: &mut String) {
        let _ = write!(out, "{}", style.paint(s));
    }

    #[inline]
    fn write_branch_a(&self, caps: &Captures<'_>, out: &mut String) {
        // YYYY sep first sep2 second  → keep order (already year-first)
        let y = caps.get(self.idx.a_year).unwrap().as_str();
        let s1 = caps.get(self.idx.a_sep1).unwrap().as_str();
        let f = caps.get(self.idx.a_first).unwrap().as_str();
        let s2 = caps.get(self.idx.a_sep2).unwrap().as_str();
        let s = caps.get(self.idx.a_second).unwrap().as_str();

        self.paint(y, &self.date, out);
        self.paint(s1, &self.separator, out);
        self.paint(f, &self.date, out);
        self.paint(s2, &self.separator, out);
        self.paint(s, &self.date, out);
    }

    #[inline]
    fn write_branch_b(&self, caps: &Captures<'_>, out: &mut String) {
        // first sep second sep2 YYYY  → normalize to year-first: YYYY sep1 first sep2 second
        let f = caps.get(self.idx.b_first).unwrap().as_str();
        let s1 = caps.get(self.idx.b_sep1).unwrap().as_str();
        let s = caps.get(self.idx.b_second).unwrap().as_str();
        let s2 = caps.get(self.idx.b_sep2).unwrap().as_str();
        let y = caps.get(self.idx.b_year).unwrap().as_str();

        self.paint(y, &self.date, out);
        self.paint(s1, &self.separator, out);
        self.paint(f, &self.date, out);
        self.paint(s2, &self.separator, out);
        self.paint(s, &self.date, out);
    }
}

impl Highlight for DateDashHighlighter {
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

            if caps.get(self.idx.a_year).is_some() {
                self.write_branch_a(&caps, &mut out);
            } else {
                self.write_branch_b(&caps, &mut out);
            }

            last = m.end();
        }
        out.push_str(&input[last..]);
        Cow::Owned(out)
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
