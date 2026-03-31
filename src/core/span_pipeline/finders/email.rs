use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct EmailFinder {
    regex: Regex,
    local_part: Style,
    at_sign: Style,
    domain: Style,
    dot: Style,
}

impl EmailFinder {
    pub fn new(local_part: Style, at_sign: Style, domain: Style, dot: Style) -> Self {
        let pattern = r"(?x)
            ([a-zA-Z0-9._%+-]+)          # local part
            (@)                           # at sign
            ([a-zA-Z0-9.-]+\.[a-zA-Z]{2,}) # domain
        ";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded email regex must compile");

        Self {
            regex,
            local_part,
            at_sign,
            domain,
            dot,
        }
    }
}

impl Finder for EmailFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'@', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            let local = caps.get(1).unwrap();
            let at = caps.get(2).unwrap();
            let domain_match = caps.get(3).unwrap();

            collector.push(local.start(), local.end(), self.local_part);
            collector.push(at.start(), at.end(), self.at_sign);

            let domain_str = domain_match.as_str();
            let domain_offset = domain_match.start();
            let mut pos = 0;
            for segment in domain_str.split('.') {
                collector.push(domain_offset + pos, domain_offset + pos + segment.len(), self.domain);
                pos += segment.len();
                if pos < domain_str.len() {
                    collector.push(domain_offset + pos, domain_offset + pos + 1, self.dot);
                    pos += 1;
                }
            }
        }
    }
}
