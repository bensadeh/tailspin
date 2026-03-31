use regex::{Error, Regex};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct RegexFinder {
    regex: Regex,
    style: Style,
}

impl RegexFinder {
    pub fn new(pattern: &str, style: Style) -> Result<Self, Error> {
        Ok(Self {
            regex: Regex::new(pattern)?,
            style,
        })
    }
}

impl Finder for RegexFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        let capture_groups = self.regex.captures_len() - 1;

        for caps in self.regex.captures_iter(input) {
            if let Some(entire_match) = caps.get(0) {
                match capture_groups {
                    1 => {
                        if let Some(captured) = caps.get(1) {
                            collector.push(captured.start(), captured.end(), self.style);
                        } else {
                            collector.push(entire_match.start(), entire_match.end(), self.style);
                        }
                    }
                    _ => {
                        collector.push(entire_match.start(), entire_match.end(), self.style);
                    }
                }
            }
        }
    }
}
