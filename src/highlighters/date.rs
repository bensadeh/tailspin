use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str| -> String { highlight_dates(&color, input) })
}

fn highlight_dates(color: &str, input: &str) -> String {
    let date_regex = Regex::new(
        r"(?x)                 # Enable comments and whitespace insensitivity
    \b                         # Word boundary, ensures we are at the start of a date/time string
    (                          # Begin capturing group for the entire date/time string
        \d{4}-\d{2}-\d{2}      # Matches date in the format: yyyy-mm-dd
        (?:                    # Begin non-capturing group for the time and timezone
            (?:\s|T)           # Matches either a whitespace or T (separator between date and time)
            \d{2}:\d{2}:\d{2}  # Matches time in the format: hh:mm:ss
            ([.,]\d+)?         # Optionally matches fractional seconds
            (Z|[+-]\d{2})?     # Optionally matches Z or timezone offset in the format: +hh or -hh
        )?                     # End non-capturing group for the time and timezone
        |                      # Alternation, matches either the pattern above or  below
        \d{2}:\d{2}:\d{2}      # Matches time in the format: hh:mm:ss
        ([.,]\d+)?             # Optionally matches fractional seconds
    )                          # End capturing group for the entire date/time string
    \b                         # Word boundary, ensures we are at the end of a date/time string
    ",
    )
    .expect("Invalid regex pattern");

    let highlighted = date_regex.replace_all(input, |caps: &regex::Captures<'_>| {
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
        let red = to_ansi(&Style {
            fg: Fg::Red,
            bg: Bg::None,
            italic: false,
            bold: false,
            underline: false,
            faint: false,
        });

        let input1 = "The time is 10:51:19.251.";
        let expected_output1 = format!("The time is {}10:51:19.251{}.", red, color::RESET);
        let input2 = "The time is 08:23:55.927.";
        let expected_output2 = format!("The time is {}08:23:55.927{}.", red, color::RESET);
        let input3 = "The date is 2022-08-29 08:11:36.";
        let expected_output3 = format!("The date is {}2022-08-29 08:11:36{}.", red, color::RESET);
        let input4 = "The date is 2022-09-22T07:46:34.171800155Z.";
        let expected_output4 = format!(
            "The date is {}2022-09-22T07:46:34.171800155Z{}.",
            red,
            color::RESET
        );
        let input5 = "The time is 08:11:36.";
        let expected_output5 = format!("The time is {}08:11:36{}.", red, color::RESET);
        let input6 = "The time is 11:48:34,534.";
        let expected_output6 = format!("The time is {}11:48:34,534{}.", red, color::RESET);
        let input7 = "The date and time are 2022-09-09 11:48:34,534.";
        let expected_output7 = format!(
            "The date and time are {}2022-09-09 11:48:34,534{}.",
            red,
            color::RESET
        );

        assert_eq!(highlight_dates(&red, input1), expected_output1);
        assert_eq!(highlight_dates(&red, input2), expected_output2);
        assert_eq!(highlight_dates(&red, input3), expected_output3);
        assert_eq!(highlight_dates(&red, input4), expected_output4);
        assert_eq!(highlight_dates(&red, input5), expected_output5);
        assert_eq!(highlight_dates(&red, input6), expected_output6);
        assert_eq!(highlight_dates(&red, input7), expected_output7);
    }
}
