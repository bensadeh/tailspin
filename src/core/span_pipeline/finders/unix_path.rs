use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct UnixPathFinder {
    regex: Regex,
    segment: Style,
    separator: Style,
}

impl UnixPathFinder {
    pub fn new(segment: Style, separator: Style) -> Self {
        let pattern = r"(?x)
            (?:^|\s)
            (?P<path>
                (?:\./|~/|//|/)
                [\w.-]+
                (?:/[\w.-]+)+
            )
        ";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded Unix path regex must compile");

        Self {
            regex,
            segment,
            separator,
        }
    }
}

impl Finder for UnixPathFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'/', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            let path_match = caps.name("path").unwrap();
            let path = path_match.as_str();
            let offset = path_match.start();

            let mut seg_start = None;

            for (i, ch) in path.char_indices() {
                if ch == '/' {
                    if let Some(start) = seg_start.take() {
                        collector.push(offset + start, offset + i, self.segment);
                    }
                    collector.push(offset + i, offset + i + 1, self.separator);
                } else if seg_start.is_none() {
                    seg_start = Some(i);
                }
            }

            if let Some(start) = seg_start {
                collector.push(offset + start, offset + path.len(), self.segment);
            }
        }
    }
}
