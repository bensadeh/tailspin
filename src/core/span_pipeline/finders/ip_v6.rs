use memchr::memchr;
use regex::{Regex, RegexBuilder};
use std::net::Ipv6Addr;

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct IpV6Finder {
    regex: Regex,
    number: Style,
    letter: Style,
    separator: Style,
}

impl IpV6Finder {
    pub fn new(number: Style, letter: Style, separator: Style) -> Self {
        let pattern = r"([0-9a-fA-F:.]{3,})(?:(/)(\d{1,3}))?";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded IPv6 regex must compile");

        Self {
            regex,
            number,
            letter,
            separator,
        }
    }
}

impl Finder for IpV6Finder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b':', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            if caps[1].parse::<Ipv6Addr>().is_ok() {
                let addr_match = caps.get(1).unwrap();
                let addr = addr_match.as_str();
                let offset = addr_match.start();

                for (i, c) in addr.char_indices() {
                    let style = match c {
                        '0'..='9' => self.number,
                        'a'..='f' | 'A'..='F' => self.letter,
                        ':' | '.' => self.separator,
                        _ => continue,
                    };
                    collector.push(offset + i, offset + i + c.len_utf8(), style);
                }

                if let (Some(slash), Some(netmask)) = (caps.get(2), caps.get(3)) {
                    collector.push(slash.start(), slash.end(), self.separator);
                    collector.push(netmask.start(), netmask.end(), self.number);
                }
            }
        }
    }
}
