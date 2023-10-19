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
            (Part::Date, "date"),
            (Part::Zone, "sep1"),
            (Part::Time, "time"),
            (Part::Time, "frac1"),
            (Part::Zone, "tz1"),
            // Or...
            (Part::Equals, "equals2"),
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

            result.join("")
        });

        highlighted.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Fg;
    use indoc::formatdoc;

    // MARK: Helpers

    fn style_hidden() -> Style {
        Style {
            hidden: true,
            ..Default::default()
        }
    }

    const RESET_ANSI: &str = color::RESET;

    // MARK: Basic functionality

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

        let test_cases = [(
            "2023-09-10T14:30:00",
            format!(
                "{}2023-09-10{}{}T{}{}14:30:00{}",
                date_ansi, RESET_ANSI, zone_ansi, RESET_ANSI, time_ansi, RESET_ANSI,
            ),
        )];

        for (input, expected_output) in test_cases.iter() {
            assert_eq!(highlighter.apply(input), *expected_output);
        }
    }

    // MARK: Hidden date tests

    #[test]
    fn test_no_hidden_fields_shows_all_fields() {
        let date_style = Style::default();
        let time_style = Style::default();
        let zone_style = Style::default();
        let date_ansi = to_ansi(&date_style);
        let time_ansi = to_ansi(&time_style);
        let zone_ansi = to_ansi(&zone_style);

        let hltr = DateHighlighter::new(&date_style, &time_style, &zone_style);
        let input = "2022-09-09 11:44:54,508 INFO test";

        // Separated by section for easier reading
        let expected: String = formatdoc!(
            "
            {date_ansi}2022-09-09{RESET_ANSI}
            {zone_ansi} {RESET_ANSI}
            {time_ansi}11:44:54{RESET_ANSI}
            {zone_ansi},508{RESET_ANSI}
             INFO test" // note space
        )
        .replace("\n", "");

        assert_eq!(hltr.apply(input), expected);
    }

    #[test]
    fn test_hidden_date_field_hides_date_field() {
        let date_style = style_hidden();
        let time_style = Style::default();
        let zone_style = Style::default();
        let time_ansi = to_ansi(&time_style);
        let zone_ansi = to_ansi(&zone_style);

        let hltr = DateHighlighter::new(&date_style, &time_style, &zone_style);
        let input = "2022-09-09 11:44:54,508 INFO test";
        let expected = formatdoc!(
            "
            {zone_ansi} {RESET_ANSI}
            {time_ansi}11:44:54{RESET_ANSI}
            {zone_ansi},508{RESET_ANSI}
             INFO test",
        )
        .replace("\n", "");

        assert_eq!(hltr.apply(input), expected);
    }

    #[test]
    fn test_hidden_time_field_hides_time_field() {
        let date_style = Style::default();
        let time_style = style_hidden();
        let zone_style = Style::default();
        let date_ansi = to_ansi(&date_style);
        let zone_ansi = to_ansi(&zone_style);

        let hltr = DateHighlighter::new(&date_style, &time_style, &zone_style);
        let input = "2022-09-09 11:44:54,508 INFO test";
        // FIXME: Is this what we want?
        let expected = formatdoc!(
            "{date_ansi}2022-09-09{RESET_ANSI}
            {zone_ansi} {RESET_ANSI}
             INFO test",
        )
        .replace("\n", "");
        assert_eq!(hltr.apply(input), expected);
    }

    #[test]
    fn test_hidden_zone_field_hides_zone_field() {
        let date_style = Style::default();
        let time_style = Style::default();
        let zone_style = style_hidden();
        let date_ansi = to_ansi(&date_style);
        let time_ansi = to_ansi(&time_style);

        let hltr = DateHighlighter::new(&date_style, &time_style, &zone_style);
        let input = "2022-09-09 11:44:54,508 INFO test";

        let expected = formatdoc!(
            "
            {date_ansi}2022-09-09{RESET_ANSI}
            {time_ansi}11:44:54{RESET_ANSI}
            {time_ansi},508{RESET_ANSI}
             INFO test",
        )
        .replace("\n", "");

        assert_eq!(hltr.apply(input), expected);
    }
}
