use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct IpV4Finder {
    regex: Regex,
    number: Style,
    separator: Style,
}

impl IpV4Finder {
    pub fn new(number: Style, separator: Style) -> Self {
        let pattern = r"(?x)\b
            (?P<o1>\d{1,3})\.
            (?P<o2>\d{1,3})\.
            (?P<o3>\d{1,3})\.
            (?P<o4>\d{1,3})
            (?:/(?P<mask>\d{1,2}))?
            \b";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded IPv4 regex must compile");

        Self {
            regex,
            number,
            separator,
        }
    }
}

impl Finder for IpV4Finder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'.', input.as_bytes()).is_none() {
            return;
        }

        let names = ["o1", "o2", "o3", "o4"];

        for caps in self.regex.captures_iter(input) {
            let valid_octets = names
                .iter()
                .all(|n| caps.name(n).unwrap().as_str().parse::<u8>().is_ok());
            let valid_mask = caps
                .name("mask")
                .is_none_or(|ms| ms.as_str().parse::<u8>().is_ok_and(|v| v <= 32));

            if valid_octets && valid_mask {
                for (i, &n) in names.iter().enumerate() {
                    let m = caps.name(n).unwrap();
                    collector.push(m.start(), m.end(), self.number);
                    if i < 3 {
                        // Push the dot separator between octets
                        collector.push(m.end(), m.end() + 1, self.separator);
                    }
                }
                if let Some(ms) = caps.name("mask") {
                    // Push the "/" separator
                    collector.push(ms.start() - 1, ms.start(), self.separator);
                    collector.push(ms.start(), ms.end(), self.number);
                }
            }
        }
    }
}
