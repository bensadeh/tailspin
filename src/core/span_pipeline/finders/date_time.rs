use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct DateTimeFinder {
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

        let mut idx = Idx {
            t: 0,
            hours: 0,
            colon1: 0,
            minutes: 0,
            colon2: 0,
            seconds: 0,
            frac_sep: 0,
            frac_digits: 0,
            tz: 0,
        };
        for (i, name) in regex.capture_names().enumerate() {
            match name {
                Some("T") => idx.t = i,
                Some("hours") => idx.hours = i,
                Some("colon1") => idx.colon1 = i,
                Some("minutes") => idx.minutes = i,
                Some("colon2") => idx.colon2 = i,
                Some("seconds") => idx.seconds = i,
                Some("frac_sep") => idx.frac_sep = i,
                Some("frac_digits") => idx.frac_digits = i,
                Some("tz") => idx.tz = i,
                _ => {}
            }
        }

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
