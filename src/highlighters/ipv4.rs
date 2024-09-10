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
    pub const fn new(number: Style, separator: Style) -> Self {
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
        IP_ADDRESS_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                format!(
                    "{}{}{}{}{}{}{}",
                    self.number.paint(&caps[1]),
                    self.separator.paint(&caps[2]),
                    self.number.paint(&caps[3]),
                    self.separator.paint(&caps[4]),
                    self.number.paint(&caps[5]),
                    self.separator.paint(&caps[6]),
                    self.number.paint(&caps[7]),
                )
            })
            .to_string()
    }
}
