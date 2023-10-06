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

#[derive(PartialEq)]
enum Part {
    Date,
    Time,
    Zone,
    Same, // For unmodified matches
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
        // Note: order matters here,
        // as this is the order the result will be formatted as to the user.
        let known_parts = [
            (Part::Same, "equals1"),
            (Part::Same, "equals2"),
            (Part::Date, "date"),
            (Part::Zone, "sep1"),
            (Part::Time, "time"),
            (Part::Time, "frac1"),
            (Part::Zone, "tz1"),
            (Part::Time, "time2"),
            (Part::Time, "frac2"),
        ];

        let highlighted = DATE_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
            let mut result: Vec<String> = Vec::new();
            for (part_type, part_name) in &known_parts {
                let Some(reg_match) = caps.name(part_name) else {
                    continue;
                };

                match part_type {
                    Part::Date => result.push(format!("{}{}{}", self.date, reg_match.as_str(), color::RESET)),
                    Part::Time => result.push(format!("{}{}{}", self.time, reg_match.as_str(), color::RESET)),
                    Part::Zone => result.push(format!("{}{}{}", self.zone, reg_match.as_str(), color::RESET)),
                    Part::Same => result.push(part_name.to_string()),
                }
            }

            result.join("")
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
