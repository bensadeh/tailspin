use memchr::memchr;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct DateTimeFinder {
    regex: Regex,
    idx: Idx,
    time: Style,
    zone: Style,
    separator: Style,
}

#[derive(Debug, Copy, Clone)]
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

impl DateTimeFinder {
    pub fn new(time: Style, zone: Style, separator: Style) -> Self {
        let pattern = r"(?x)
            (?P<T>[T\s])?
            (?P<hours>[01]?\d|2[0-3])(?P<colon1>:)
            (?P<minutes>[0-5]\d)(?P<colon2>:)
            (?P<seconds>[0-5]\d)
            (?P<frac_sep>[.,:])?(?P<frac_digits>\d+)?
            (?P<tz>Z)?
        ";

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded date-time regex must compile");

        let mut name_to_idx = HashMap::new();
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

        Self {
            regex,
            idx,
            time,
            zone,
            separator,
        }
    }

    fn push_part(collector: &mut Collector, caps: &regex::Captures<'_>, i: usize, style: Style) {
        if let Some(m) = caps.get(i) {
            collector.push(m.start(), m.end(), style);
        }
    }
}

impl Finder for DateTimeFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b':', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            Self::push_part(collector, &caps, self.idx.t, self.zone);
            Self::push_part(collector, &caps, self.idx.hours, self.time);
            Self::push_part(collector, &caps, self.idx.colon1, self.separator);
            Self::push_part(collector, &caps, self.idx.minutes, self.time);
            Self::push_part(collector, &caps, self.idx.colon2, self.separator);
            Self::push_part(collector, &caps, self.idx.seconds, self.time);
            Self::push_part(collector, &caps, self.idx.frac_sep, self.separator);
            Self::push_part(collector, &caps, self.idx.frac_digits, self.time);
            Self::push_part(collector, &caps, self.idx.tz, self.zone);
        }
    }
}
