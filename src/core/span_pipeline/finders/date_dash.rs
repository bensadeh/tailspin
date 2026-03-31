use memchr::memchr2;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

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

        let mut map = HashMap::new();
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
