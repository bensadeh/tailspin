use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct IpV4Finder {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> IpV4Finder {
        IpV4Finder::new(Style::new().fg(Color::Blue), Style::new().fg(Color::Red))
    }

    fn span_texts<'a>(input: &'a str, finder: &IpV4Finder) -> Vec<&'a str> {
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn valid_ipv4() {
        let texts = span_texts("10.0.0.123", &make_finder());
        assert_eq!(texts, ["10", ".", "0", ".", "0", ".", "123"]);
    }

    #[test]
    fn ipv4_with_cidr() {
        let texts = span_texts("192.168.0.1/24", &make_finder());
        assert!(texts.contains(&"192"));
        assert!(texts.contains(&"1"));
        assert!(texts.contains(&"/"));
        assert!(texts.contains(&"24"));
    }

    #[test]
    fn all_zeros() {
        let texts = span_texts("0.0.0.0", &make_finder());
        assert_eq!(texts, ["0", ".", "0", ".", "0", ".", "0"]);
    }

    #[test]
    fn octet_over_255_no_match() {
        assert!(span_texts("256.1.1.1", &make_finder()).is_empty());
    }

    #[test]
    fn all_999_no_match() {
        assert!(span_texts("999.999.999.999", &make_finder()).is_empty());
    }

    #[test]
    fn mask_over_32_no_match() {
        assert!(span_texts("192.168.0.1/33", &make_finder()).is_empty());
    }

    #[test]
    fn partial_address_no_match() {
        assert!(span_texts("1.2.3", &make_finder()).is_empty());
    }
}
