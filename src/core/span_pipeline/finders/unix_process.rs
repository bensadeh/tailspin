use memchr::memchr;
use regex::{Regex, RegexBuilder};

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct UnixProcessFinder {
    regex: Regex,
    name: Style,
    id: Style,
    bracket: Style,
}

impl UnixProcessFinder {
    pub fn new(name: Style, id: Style, bracket: Style) -> Self {
        let pattern = r"(?P<process_name>\([A-Za-z0-9._ +:/-]+\)|[A-Za-z0-9_/-]+)\[(?P<process_id>\d+)]";
        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded Unix process regex must compile");

        Self {
            regex,
            name,
            id,
            bracket,
        }
    }
}

impl Finder for UnixProcessFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        if memchr(b'[', input.as_bytes()).is_none() {
            return;
        }

        for caps in self.regex.captures_iter(input) {
            if let Some(p) = caps.name("process_name") {
                collector.push(p.start(), p.end(), self.name);
            }
            // The '[' is between process_name end and process_id start
            let pid = caps.name("process_id").unwrap();
            collector.push(pid.start() - 1, pid.start(), self.bracket); // [
            collector.push(pid.start(), pid.end(), self.id);
            collector.push(pid.end(), pid.end() + 1, self.bracket); // ]
        }
    }
}
