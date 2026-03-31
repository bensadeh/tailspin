use memchr::memchr2;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct DateDashFinder {
    regex: Regex,
    idx: Idx,
    date: Style,
    separator: Style,
}

#[derive(Debug, Copy, Clone)]
struct Idx {
    a_year: usize,
    a_sep1: usize,
    a_first: usize,
    a_sep2: usize,
    a_second: usize,
    b_first: usize,
    b_sep1: usize,
    b_second: usize,
    b_sep2: usize,
    b_year: usize,
}

impl DateDashFinder {
    pub fn new(date: Style, separator: Style) -> Self {
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

        let mut idx = Idx {
            a_year: 0,
            a_sep1: 0,
            a_first: 0,
            a_sep2: 0,
            a_second: 0,
            b_first: 0,
            b_sep1: 0,
            b_second: 0,
            b_sep2: 0,
            b_year: 0,
        };
        for (i, name) in regex.capture_names().enumerate() {
            match name {
                Some("a_year") => idx.a_year = i,
                Some("a_sep1") => idx.a_sep1 = i,
                Some("a_first") => idx.a_first = i,
                Some("a_sep2") => idx.a_sep2 = i,
                Some("a_second") => idx.a_second = i,
                Some("b_first") => idx.b_first = i,
                Some("b_sep1") => idx.b_sep1 = i,
                Some("b_second") => idx.b_second = i,
                Some("b_sep2") => idx.b_sep2 = i,
                Some("b_year") => idx.b_year = i,
                _ => {}
            }
        }

        Self {
            regex,
            idx,
            date,
            separator,
        }
    }
}

impl Finder for DateDashFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr2(b'-', b'/', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            if caps.get(self.idx.a_year).is_some() {
                // Branch A: YYYY-xx-xx — spans at original positions
                for &i in &[self.idx.a_year, self.idx.a_first, self.idx.a_second] {
                    let m = caps.get(i).unwrap();
                    collector.push(m.start(), m.end(), self.date);
                }
                for &i in &[self.idx.a_sep1, self.idx.a_sep2] {
                    let m = caps.get(i).unwrap();
                    collector.push(m.start(), m.end(), self.separator);
                }
            } else {
                // Branch B: xx-xx-YYYY — spans at original positions
                for &i in &[self.idx.b_first, self.idx.b_second, self.idx.b_year] {
                    let m = caps.get(i).unwrap();
                    collector.push(m.start(), m.end(), self.date);
                }
                for &i in &[self.idx.b_sep1, self.idx.b_sep2] {
                    let m = caps.get(i).unwrap();
                    collector.push(m.start(), m.end(), self.separator);
                }
            }
        }
    }
}
