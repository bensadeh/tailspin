use super::RegexExt;
use crate::core::config::DateTimeConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use memchr::memchr2;
use regex::{Captures, Regex, RegexBuilder};
use std::borrow::Cow;

pub struct DateDashHighlighter {
    regex: Regex,
    date: Painter,
    separator: Painter,
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
    pub fn new(time_config: DateTimeConfig) -> Self {
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

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded date-dash regex must compile");

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

        Self {
            regex,
            idx,
            date: Painter::new(time_config.date.into()),
            separator: Painter::new(time_config.separator.into()),
        }
    }

    #[inline]
    fn write_branch_a(&self, caps: &Captures<'_>, out: &mut String) {
        // YYYY sep first sep2 second  → keep order (already year-first)
        let y = caps.get(self.idx.a_year).unwrap().as_str();
        let s1 = caps.get(self.idx.a_sep1).unwrap().as_str();
        let f = caps.get(self.idx.a_first).unwrap().as_str();
        let s2 = caps.get(self.idx.a_sep2).unwrap().as_str();
        let s = caps.get(self.idx.a_second).unwrap().as_str();

        self.date.paint(out, y);
        self.separator.paint(out, s1);
        self.date.paint(out, f);
        self.separator.paint(out, s2);
        self.date.paint(out, s);
    }

    #[inline]
    fn write_branch_b(&self, caps: &Captures<'_>, out: &mut String) {
        // first sep second sep2 YYYY  → normalize to year-first: YYYY sep1 first sep2 second
        let f = caps.get(self.idx.b_first).unwrap().as_str();
        let s1 = caps.get(self.idx.b_sep1).unwrap().as_str();
        let s = caps.get(self.idx.b_second).unwrap().as_str();
        let s2 = caps.get(self.idx.b_sep2).unwrap().as_str();
        let y = caps.get(self.idx.b_year).unwrap().as_str();

        self.date.paint(out, y);
        self.separator.paint(out, s1);
        self.date.paint(out, f);
        self.separator.paint(out, s2);
        self.date.paint(out, s);
    }
}

impl Highlight for DateDashHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr2(b'-', b'/', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            if caps.get(self.idx.a_year).is_some() {
                self.write_branch_a(caps, buf);
            } else {
                self.write_branch_b(caps, buf);
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
    fn test_date_dash_highlighter() {
        let config = DateTimeConfig {
            date: Style::new().fg(Color::Magenta),
            separator: Style::new().fg(Color::Blue),
            ..DateTimeConfig::default()
        };
        let highlighter = DateDashHighlighter::new(config);

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
