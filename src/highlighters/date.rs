use crate::color;
use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regex::DATE_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct DateHighlighter {
    date: String,
    time: String,
    zone: String,
}

impl DateHighlighter {
    pub fn new(date: &Style, time: &Style, zone: &Style) -> Self {
        Self {
            date: to_ansi(date),
            time: to_ansi(time),
            zone: to_ansi(zone),
        }
    }
}

impl Highlight for DateHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.dashes < 2 && line_info.colons < 2 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        let highlighted = DATE_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
            let date_part = if let Some(m) = caps.name("date") {
                format!("{}{}{}", self.date, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let sep1_part = if let Some(m) = caps.name("sep1") {
                format!("{}{}{}", self.zone, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let time_part = if let Some(m) = caps.name("time") {
                format!("{}{}{}", self.time, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let frac1_part = if let Some(m) = caps.name("frac1") {
                format!("{}{}{}", self.time, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let tz1_part = if let Some(m) = caps.name("tz1") {
                format!("{}{}{}", self.zone, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let time2_part = if let Some(m) = caps.name("time2") {
                format!("{}{}{}", self.time, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            let frac2_part = if let Some(m) = caps.name("frac2") {
                format!("{}{}{}", self.time, m.as_str(), color::RESET)
            } else {
                String::new()
            };

            format!(
                "{}{}{}{}{}{}{}",
                date_part, sep1_part, time_part, frac1_part, tz1_part, time2_part, frac2_part
            )
        });

        highlighted.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Fg;

    #[test]
    fn test_highlight_dates() {
        let date = Style {
            fg: Fg::Red,
            ..Default::default()
        };

        let time = Style {
            fg: Fg::Green,
            ..Default::default()
        };
        let zone = Style {
            fg: Fg::Blue,
            ..Default::default()
        };

        let highlighter = DateHighlighter::new(&date, &time, &zone);
        let date_ansi = to_ansi(&date);
        let time_ansi = to_ansi(&time);
        let zone_ansi = to_ansi(&zone);
        let reset_ansi = color::RESET;

        let test_cases = [(
            "2023-09-10T14:30:00",
            format!(
                "{}2023-09-10{}{}T{}{}14:30:00{}",
                date_ansi, reset_ansi, zone_ansi, reset_ansi, time_ansi, reset_ansi,
            ),
        )];

        for (input, expected_output) in test_cases.iter() {
            assert_eq!(highlighter.apply(input), *expected_output);
        }
    }
}
