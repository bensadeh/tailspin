use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static DATE_WORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?x)                                         
        (?P<day1>\b(?:Mon|Tue|Wed|Thu|Fri|Sat|Sun)\b)? 
        \s* 
        (?P<month>\b(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\b) 
        \s+ 
        (?P<day2>\b(?:[0-2]?[0-9]|3[0-1])\b)   
    "#,
    )
    .expect("Invalid regex pattern")
});

pub struct DateWordHighlighter {
    day_name: Style,
    month_name: Style,
    day_number: Style,
}

impl DateWordHighlighter {
    pub const fn new(day_name: Style, month_name: Style, day_number: Style) -> Self {
        Self {
            day_name,
            month_name,
            day_number,
        }
    }
}

impl Highlight for DateWordHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        DATE_WORD_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                let day1 = caps.name("day1").map(|m| m.as_str());
                let month = caps.name("month").map(|m| m.as_str());
                let day2 = caps.name("day2").map(|m| m.as_str());

                let formatted_day1 = match day1 {
                    Some(d1) => format!("{} ", self.day_name.paint(d1)),
                    None => "".to_string(),
                };

                match (month, day2) {
                    (Some(mo), Some(d2)) => format!(
                        "{}{} {}",
                        formatted_day1,
                        self.month_name.paint(mo),
                        self.day_number.paint(d2)
                    ),
                    _ => input.to_string(),
                }
            })
            .to_string()
    }
}
