use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static IP_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\b\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3})(\.)(\d{1,3}\b)").expect("Invalid IP address regex pattern")
});

pub struct Ipv4Highlighter {
    number: Style,
    separator: Style,
}

impl Ipv4Highlighter {
    pub fn new(number: Style, separator: Style) -> Self {
        Self { number, separator }
    }
}

impl Highlight for Ipv4Highlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.dots < 3
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        let segment = &self.number;
        let separator = &self.separator;
        let highlight_groups = [
            (segment, 1),
            (separator, 2),
            (segment, 3),
            (separator, 4),
            (segment, 5),
            (separator, 6),
            (segment, 7),
        ];

        IP_ADDRESS_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                let mut output = String::new();
                for &(color, group) in &highlight_groups {
                    output.push_str(&format!("{}", color.paint(&caps[group])));
                }
                output
            })
            .to_string()
    }
}
