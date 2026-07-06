use super::build_regex;
use memchr::memchr;
use regex::Regex;

use crate::core::config::EmailConfig;

use super::super::span::{Collector, Finder};

#[derive(Debug, Clone)]
pub(crate) struct EmailFinder {
    regex: Regex,
    config: EmailConfig,
}

impl EmailFinder {
    pub fn new(config: EmailConfig) -> Self {
        // Match structure: local@domain — we find '@' in the match to split the parts.
        let pattern = r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}";
        let regex = build_regex(pattern);

        Self { regex, config }
    }
}

impl Finder for EmailFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'@', input.as_bytes()).is_none() {
            return;
        }

        let EmailConfig {
            local_part,
            at_sign,
            domain,
            dot,
        } = self.config;

        for m in self.regex.find_iter(input) {
            let s = m.start();
            let bytes = m.as_str().as_bytes();
            let at = memchr(b'@', bytes).unwrap();

            collector.push(s, s + at, local_part);
            collector.push(s + at, s + at + 1, at_sign);

            // Domain: highlight segments and dots separately
            let domain_offset = s + at + 1;
            let domain_bytes = &bytes[at + 1..];
            let mut pos = 0;
            for segment in domain_bytes.split(|&b| b == b'.') {
                if segment.is_empty() {
                    pos += 1;
                    continue;
                }
                collector.push(domain_offset + pos, domain_offset + pos + segment.len(), domain);
                pos += segment.len();
                if pos < domain_bytes.len() {
                    collector.push(domain_offset + pos, domain_offset + pos + 1, dot);
                    pos += 1;
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

    fn make_finder() -> EmailFinder {
        EmailFinder::new(EmailConfig {
            local_part: Style::new().fg(Color::Cyan),
            at_sign: Style::new().fg(Color::Red),
            domain: Style::new().fg(Color::Green),
            dot: Style::new().fg(Color::Yellow),
        })
    }

    #[test]
    fn finds_email() {
        let texts = span_texts("contact user@example.com today", &make_finder());
        assert_eq!(texts, ["user", "@", "example", ".", "com"]);
    }

    #[test]
    fn email_with_plus_and_subdomains() {
        let texts = span_texts("first.last+tag@sub.domain.co.uk", &make_finder());
        assert_eq!(
            texts,
            ["first.last+tag", "@", "sub", ".", "domain", ".", "co", ".", "uk"]
        );
    }

    #[test]
    fn multiple_emails() {
        let texts = span_texts("From alice@a.com to bob@b.org", &make_finder());
        assert!(texts.contains(&"alice"));
        assert!(texts.contains(&"bob"));
    }

    #[test]
    fn no_email_no_match() {
        assert!(span_texts("No email here!", &make_finder()).is_empty());
    }

    #[test]
    fn double_dot_domain_does_not_panic() {
        let finder = make_finder();
        let mut collector = Collector::new();
        // a..com has consecutive dots — should not panic in any build mode
        finder.find_spans("user@a..com", &mut collector);

        let spans = collector.into_spans();
        // Should produce spans without panicking.
        // The consecutive dot is skipped rather than producing a zero-width span.
        assert!(spans.iter().all(|s| s.start < s.end));
    }
}
