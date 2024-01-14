use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

static TIME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)                       
                (?:
                    (?P<T>[T\s])?               # Capture separator (either a space or T)
                    (?P<time>\d{2}:\d{2}:\d{2}) # Capture time alone
                    (?P<frac>[.,]\d+)?          # Capture fractional seconds
                    (?P<tz>Z)?                  # Capture timezone (Zulu time)
                )  
    ",
    )
    .expect("Invalid regex pattern")
});

pub struct TimeHighlighter {
    time: Style,
    zone: Style,
}

impl TimeHighlighter {
    pub fn new(time: Style, zone: Style) -> Self {
        Self { time, zone }
    }

    fn highlight_time(&self, input: &str) -> String {
        let highlighted = TIME_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
            let t_part = if let Some(m) = caps.name("T") {
                format!("{}", self.zone.paint(m.as_str()))
            } else {
                String::new()
            };

            let time_part = if let Some(m) = caps.name("time") {
                format!("{}", self.time.paint(m.as_str()))
            } else {
                String::new()
            };

            let frac_part = if let Some(m) = caps.name("frac") {
                format!("{}", self.time.paint(m.as_str()))
            } else {
                String::new()
            };

            let zone_part = if let Some(m) = caps.name("tz") {
                format!("{}", self.zone.paint(m.as_str()))
            } else {
                String::new()
            };

            format!("{}{}{}{}", t_part, time_part, frac_part, zone_part)
        });

        highlighted.into_owned()
    }
}

impl Highlight for TimeHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.colons < 2
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        self.highlight_time(input)
    }
}
