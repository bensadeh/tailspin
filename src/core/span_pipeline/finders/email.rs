use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct EmailFinder {
    regex: Regex,
    local_part: Style,
    at_sign: Style,
    domain: Style,
    dot: Style,
}

impl EmailFinder {
    pub fn new(local_part: Style, at_sign: Style, domain: Style, dot: Style) -> Self {
        // Match structure: local@domain — we find '@' in the match to split the parts.
        let pattern = r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}";
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

        for m in self.regex.find_iter(input) {
            let s = m.start();
            let bytes = m.as_str().as_bytes();
            let at = memchr(b'@', bytes).unwrap();

            collector.push(s, s + at, self.local_part);
            collector.push(s + at, s + at + 1, self.at_sign);

            // Domain: highlight segments and dots separately
            let domain_offset = s + at + 1;
            let domain_bytes = &bytes[at + 1..];
            let mut pos = 0;
            for segment in domain_bytes.split(|&b| b == b'.') {
                if segment.is_empty() {
                    pos += 1;
                    continue;
                }
                collector.push(domain_offset + pos, domain_offset + pos + segment.len(), self.domain);
                pos += segment.len();
                if pos < domain_bytes.len() {
                    collector.push(domain_offset + pos, domain_offset + pos + 1, self.dot);
                    pos += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> EmailFinder {
        EmailFinder::new(
            Style::new().fg(Color::Cyan),
            Style::new().fg(Color::Red),
            Style::new().fg(Color::Green),
            Style::new().fg(Color::Yellow),
        )
    }

    #[test]
    fn finds_email() {
        let finder = make_finder();
        let mut collector = Collector::new(0);
        finder.find_spans("contact user@example.com today", &mut collector);

        let (spans, _) = collector.into_parts();
        // local("user") + at("@") + domain("example") + dot(".") + domain("com")
        assert_eq!(spans.len(), 5);
        assert_eq!(&"contact user@example.com today"[spans[0].start..spans[0].end], "user");
        assert_eq!(&"contact user@example.com today"[spans[1].start..spans[1].end], "@");
        assert_eq!(
            &"contact user@example.com today"[spans[2].start..spans[2].end],
            "example"
        );
        assert_eq!(&"contact user@example.com today"[spans[3].start..spans[3].end], ".");
        assert_eq!(&"contact user@example.com today"[spans[4].start..spans[4].end], "com");
    }

    #[test]
    fn email_with_plus_and_subdomains() {
        let finder = make_finder();
        let input = "first.last+tag@sub.domain.co.uk";
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);

        let (spans, _) = collector.into_parts();
        assert_eq!(&input[spans[0].start..spans[0].end], "first.last+tag");
        assert_eq!(&input[spans[1].start..spans[1].end], "@");
        // Domain parts: sub, ., domain, ., co, ., uk
        assert_eq!(spans.len(), 9);
    }

    #[test]
    fn multiple_emails() {
        let finder = make_finder();
        let input = "From alice@a.com to bob@b.org";
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);

        let (spans, _) = collector.into_parts();
        let texts: Vec<&str> = spans.iter().map(|s| &input[s.start..s.end]).collect();
        assert!(texts.contains(&"alice"));
        assert!(texts.contains(&"bob"));
    }

    #[test]
    fn no_email_no_match() {
        let finder = make_finder();
        let mut collector = Collector::new(0);
        finder.find_spans("No email here!", &mut collector);
        assert!(collector.into_spans().is_empty());
    }

    #[test]
    fn double_dot_domain_does_not_panic() {
        let finder = make_finder();
        let mut collector = Collector::new(0);
        // a..com has consecutive dots — should not panic in any build mode
        finder.find_spans("user@a..com", &mut collector);

        let (spans, _) = collector.into_parts();
        // Should produce spans without panicking.
        // The consecutive dot is skipped rather than producing a zero-width span.
        assert!(spans.iter().all(|s| s.start < s.end));
    }
}
