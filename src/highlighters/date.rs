use crate::color;
use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regex::DATE_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct DateHighlighter {
    date_ansi: Option<String>,
    time_ansi: Option<String>,
    zone_ansi: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Part {
    Date,
    Time,
    Zone,
    Equals,
}

impl DateHighlighter {
    pub fn new(date: &Style, time: &Style, zone: &Style) -> Self {
        Self {
            date_ansi: (!date.hidden).then_some(to_ansi(date)),
            time_ansi: (!time.hidden).then_some(to_ansi(time)),
            zone_ansi: (!zone.hidden).then_some(to_ansi(zone)),
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
        let named_captures = [
            (Part::Equals, "equals1"),
            (Part::Equals, "equals2"),
            (Part::Date, "date"),
            (Part::Zone, "sep1"),
            (Part::Time, "time"),
            (Part::Time, "frac1"),
            (Part::Zone, "tz1"),
            (Part::Time, "time2"),
            (Part::Time, "frac2"),
        ];

        let highlighted = DATE_REGEX.replace_all(input, |captures: &regex::Captures<'_>| {
            let mut result: Vec<String> = Vec::new();
            for (part_type, part_name) in &named_captures {
                // Only process known regex captures
                let Some(reg_match) = captures.name(part_name) else {
                    continue;
                };

                match part_type {
                    Part::Date => {
                        if let Some(date) = &self.date_ansi {
                            result.push(format!("{}{}{}", date, reg_match.as_str(), color::RESET))
                        }
                    }
                    Part::Time => {
                        if let Some(time) = &self.time_ansi {
                            result.push(format!("{}{}{}", time, reg_match.as_str(), color::RESET))
                        }
                    }
                    Part::Zone => {
                        if let Some(zone) = &self.zone_ansi {
                            result.push(format!("{}{}{}", zone, reg_match.as_str(), color::RESET))
                        }
                    }
                    // TODO: not sure how to handle the `same` case
                    Part::Equals => result.push(format!("{}", reg_match.as_str())),
                }
            }

            // println!("result.join is {:#?}", result.join(""));
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
    fn test_show_all_date_fields_displays_all_fields() {
        let date = Style::default();
        let time = Style::default();
        let zone = Style::default();
        let date_ansi = to_ansi(&date);
        let time_ansi = to_ansi(&time);
        let zone_ansi = to_ansi(&zone);
        let reset_ansi = color::RESET;

        let hltr = DateHighlighter::new(&date, &time, &zone);
        let input = "2022-09-09 11:44:54,508 INFO test";

        // Note: space is the zone character
        let expected = format!(
            "{}2022-09-09{}{} {}{}11:44:54{}{},508{} INFO test",
            date_ansi, reset_ansi, zone_ansi, reset_ansi, time_ansi, reset_ansi, zone_ansi, reset_ansi
        );

        assert_eq!(hltr.apply(input), expected);
    }

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
