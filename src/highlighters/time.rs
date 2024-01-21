use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

static TIME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
            \b                                         # Word boundary, ensures we are at the start of a time
            (?P<T>[T\s])?                              # Capture separator (either a space or T)
            (?P<hours>\d{2})(?P<colon1>:)
            (?P<minutes>\d{2})(?P<colon2>:)
            (?P<seconds>\d{2})
            (?P<frac_sep>[.,:])?(?P<frac_digits>\d+)?  # Capture fractional seconds (separator and digits separately)
            (?P<tz>Z)?                                 # Capture timezone (Z)
    ",
    )
    .expect("Invalid regex pattern")
});

pub struct TimeHighlighter {
    time: Style,
    zone: Style,
    separator: Style,
}

impl TimeHighlighter {
    pub fn new(time: Style, zone: Style, separator: Style) -> Self {
        Self { time, zone, separator }
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
        TIME_REGEX
            .replace_all(input, |caps: &regex::Captures<'_>| {
                let paint_and_stringify = |name: &str, style: &Style| {
                    caps.name(name)
                        .map(|m| style.paint(m.as_str()).to_string())
                        .unwrap_or_default()
                };

                let parts = [
                    ("T", &self.zone),
                    ("hours", &self.time),
                    ("colon1", &self.separator),
                    ("minutes", &self.time),
                    ("colon2", &self.separator),
                    ("seconds", &self.time),
                    ("frac_sep", &self.separator),
                    ("frac_digits", &self.time),
                    ("tz", &self.zone),
                ];

                parts.iter().fold(String::new(), |acc, (name, style)| {
                    acc + &paint_and_stringify(name, style)
                })
            })
            .to_string()
    }
}
