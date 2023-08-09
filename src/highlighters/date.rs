use crate::color;
use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regexes::DATE_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct DateHighlighter {
    color: String,
}

impl DateHighlighter {
    pub fn new(style: &Style) -> Self {
        Self {
            color: to_ansi(style),
        }
    }
}

impl Highlight for DateHighlighter {
    fn apply(&self, input: &str, line_info: &LineInfo) -> String {
        highlight_dates(&self.color, input, line_info)
    }
}

fn highlight_dates(color: &str, input: &str, line_info: &LineInfo) -> String {
    // if line does not have at least two dashes or two colons, it is not a date
    if line_info.dashes < 2 && line_info.colons < 2 {
        return input.to_string();
    }

    let highlighted = DATE_REGEX.replace_all(input, |caps: &regex::Captures<'_>| {
        format!("{}{}{}", color, &caps[0], color::RESET)
    });

    highlighted.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Bg, Fg};

    #[test]
    fn test_highlight_dates() {
        let style = Style {
            fg: Fg::Red,
            bg: Bg::None,
            italic: false,
            bold: false,
            underline: false,
            faint: false,
        };
        let highlighter = DateHighlighter::new(&style);

        let line_info = LineInfo {
            dashes: 2,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colons: 2,
        };

        let red = to_ansi(&style);

        let test_cases = [
            (
                "The time is 10:51:19.251.",
                format!("The time is {}10:51:19.251{}.", red, color::RESET),
            ),
            (
                "The time is 08:23:55.927.",
                format!("The time is {}08:23:55.927{}.", red, color::RESET),
            ),
            (
                "The date is 2022-08-29 08:11:36.",
                format!("The date is {}2022-08-29 08:11:36{}.", red, color::RESET),
            ),
            (
                "The date is 2022-09-22T07:46:34.171800155Z.",
                format!(
                    "The date is {}2022-09-22T07:46:34.171800155Z{}.",
                    red,
                    color::RESET
                ),
            ),
            (
                "The time is 08:11:36.",
                format!("The time is {}08:11:36{}.", red, color::RESET),
            ),
            (
                "The time is 11:48:34,534.",
                format!("The time is {}11:48:34,534{}.", red, color::RESET),
            ),
            (
                "The date and time are 2022-09-09 11:48:34,534.",
                format!(
                    "The date and time are {}2022-09-09 11:48:34,534{}.",
                    red,
                    color::RESET
                ),
            ),
        ];

        for (input, expected_output) in test_cases.iter() {
            assert_eq!(highlighter.apply(input, &line_info), *expected_output);
        }
    }
}
