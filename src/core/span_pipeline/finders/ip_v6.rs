use super::build_regex;
use memchr::memchr;
use regex::Regex;
use std::net::Ipv6Addr;

use crate::core::config::IpV6Config;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct IpV6Finder {
    regex: Regex,
    config: IpV6Config,
}

impl IpV6Finder {
    pub fn new(config: IpV6Config) -> Self {
        let pattern = r"([0-9a-fA-F:.]{3,})(?:(/)(\d{1,3}))?";
        let regex = build_regex(pattern);

        Self { regex, config }
    }
}

impl Finder for IpV6Finder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b':', input.as_bytes()).is_none() {
            return;
        }

        let IpV6Config {
            number,
            letter,
            separator,
        } = self.config;

        for caps in self.regex.captures_iter(input) {
            let valid_addr = caps[1].parse::<Ipv6Addr>().is_ok();
            let valid_mask = caps
                .get(3)
                .is_none_or(|m| m.as_str().parse::<u8>().is_ok_and(|v| v <= 128));

            if valid_addr && valid_mask {
                let addr_match = caps.get(1).unwrap();
                let addr = addr_match.as_str();
                let offset = addr_match.start();

                for (i, c) in addr.char_indices() {
                    let style = match c {
                        '0'..='9' => number,
                        'a'..='f' | 'A'..='F' => letter,
                        ':' | '.' => separator,
                        _ => continue,
                    };
                    collector.push(offset + i, offset + i + c.len_utf8(), style);
                }

                if let (Some(slash), Some(netmask)) = (caps.get(2), caps.get(3)) {
                    collector.push(slash.start(), slash.end(), separator);
                    collector.push(netmask.start(), netmask.end(), number);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::{Color, Style};

    fn make_finder() -> IpV6Finder {
        IpV6Finder::new(IpV6Config {
            number: Style::new().fg(Color::Blue),
            letter: Style::new().fg(Color::Yellow),
            separator: Style::new().fg(Color::Red),
        })
    }

    fn span_count(input: &str) -> usize {
        let mut collector = Collector::new();
        make_finder().find_spans(input, &mut collector);
        collector.into_spans().len()
    }

    fn matched_range(input: &str) -> Option<(usize, usize)> {
        let mut collector = Collector::new();
        make_finder().find_spans(input, &mut collector);
        let spans = collector.into_spans();
        if spans.is_empty() {
            None
        } else {
            Some((spans.first().unwrap().start, spans.last().unwrap().end))
        }
    }

    #[test]
    fn full_ipv6() {
        assert!(span_count("2001:db8:0:0:0:ff00:42:8329") > 0);
    }

    #[test]
    fn compressed_ipv6() {
        assert!(span_count("2001:db8::ff00:42:8329") > 0);
    }

    #[test]
    fn loopback() {
        assert!(span_count("::1") > 0);
    }

    #[test]
    fn ipv4_mapped() {
        assert!(span_count("::ffff:127.0.0.1") > 0);
    }

    #[test]
    fn cidr_notation() {
        let input = "fe80::/10";
        let (start, end) = matched_range(input).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, input.len());
    }

    #[test]
    fn mask_over_128_no_match() {
        assert_eq!(span_count("fe80::/129"), 0);
    }

    #[test]
    fn dual_stack_ipv4_in_ipv6() {
        assert!(span_count("2001:db8:85a3::8a2e:192.0.2.33") > 0);
    }

    #[test]
    fn plain_ipv4_no_match() {
        assert_eq!(span_count("Not ipv4: 192.168.0.1"), 0);
    }

    #[test]
    fn time_like_no_match() {
        assert_eq!(span_count("11:47:39:850"), 0);
    }

    #[test]
    fn slash_separated_no_match() {
        assert_eq!(span_count("123/234/345/456"), 0);
    }
}
