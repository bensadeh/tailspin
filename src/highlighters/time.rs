use crate::color;
use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regex::TIME_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct TimeHighlighter {
    time: String,
    zone: String,
}

impl TimeHighlighter {
    pub fn new(time: &Style, zone: &Style) -> Self {
        Self {
            time: to_ansi(time),
            zone: to_ansi(zone),
        }
    }
}

impl Highlight for TimeHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.colons < 2 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        let highlighted = TIME_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
            let t_part = if let Some(m) = caps.name("T") {
                format!("{}{}{}", self.zone, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let time_part = if let Some(m) = caps.name("time") {
                format!("{}{}{}", self.time, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let frac_part = if let Some(m) = caps.name("frac") {
                format!("{}{}{}", self.time, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let zone_part = if let Some(m) = caps.name("tz") {
                format!("{}{}{}", self.zone, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            format!("{}{}{}{}", t_part, time_part, frac_part, zone_part)
        });

        highlighted.into_owned()
    }
}
