use super::build_regex;
use memchr::memchr;
use regex::Regex;

use crate::core::config::IpV4Config;

use super::super::palette::{Palette, StyleId};
use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct IpV4Finder {
    regex: Regex,
    number: StyleId,
    separator: StyleId,
}

impl IpV4Finder {
    pub fn new(config: IpV4Config, palette: &mut Palette) -> Self {
        let pattern = r"(?x)\b
            (?P<o1>\d{1,3})(?P<d1>\.)
            (?P<o2>\d{1,3})(?P<d2>\.)
            (?P<o3>\d{1,3})(?P<d3>\.)
            (?P<o4>\d{1,3})
            (?:(?P<slash>/)(?P<mask>\d{1,2}))?
            \b";
        let regex = build_regex(pattern);

        Self {
            regex,
            number: palette.intern(config.number),
            separator: palette.intern(config.separator),
        }
    }
}

impl Finder for IpV4Finder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'.', input.as_bytes()).is_none() {
            return;
        }

        let octets = ["o1", "o2", "o3", "o4"];
        let dots = ["d1", "d2", "d3"];

        for caps in self.regex.captures_iter(input) {
            let valid_octets = octets
                .iter()
                .all(|n| caps.name(n).unwrap().as_str().parse::<u8>().is_ok());
            let valid_mask = caps
                .name("mask")
                .is_none_or(|ms| ms.as_str().parse::<u8>().is_ok_and(|v| v <= 32));

            if valid_octets && valid_mask {
                for (i, &name) in octets.iter().enumerate() {
                    let octet = caps.name(name).unwrap();
                    collector.push(octet.start(), octet.end(), self.number);
                    if let Some(dot) = dots.get(i).and_then(|&d| caps.name(d)) {
                        collector.push(dot.start(), dot.end(), self.separator);
                    }
                }
                if let Some(slash) = caps.name("slash") {
                    collector.push(slash.start(), slash.end(), self.separator);
                    let mask = caps.name("mask").unwrap();
                    collector.push(mask.start(), mask.end(), self.number);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::span_texts;
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> IpV4Finder {
        IpV4Finder::new(
            IpV4Config {
                number: Style::new().fg(Color::Blue),
                separator: Style::new().fg(Color::Red),
            },
            &mut Palette::new(),
        )
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
