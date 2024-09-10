use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static POINTER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?ix)
            \b(?P<prefix>0x)(?P<first_half>[0-9a-fA-F]{8})\b          
            |
            \b(?P<prefix64>0x)(?P<first_half64>[0-9a-fA-F]{8})(?P<second_half>[0-9a-fA-F]{8})\b  
        ",
    )
    .expect("Invalid pointer regex pattern")
});

pub struct PointerHighlighter {
    number: Style,
    letter: Style,
    separator: Style,
    separator_token: char,
    x: Style,
}

impl PointerHighlighter {
    pub const fn new(number: Style, letter: Style, separator: Style, separator_token: char, x: Style) -> Self {
        Self {
            number,
            letter,
            separator,
            separator_token,
            x,
        }
    }
}

impl Highlight for PointerHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.x < 1 && line_info.zeros < 1
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        POINTER_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                let prefix = caps.name("prefix").or_else(|| caps.name("prefix64")).unwrap().as_str();
                let first_half = caps
                    .name("first_half")
                    .or_else(|| caps.name("first_half64"))
                    .unwrap()
                    .as_str();
                let formatted_prefix = prefix
                    .chars()
                    .map(|c| highlight_char(c, self.number, self.x, self.letter))
                    .collect::<String>();
                let formatted_first_half = first_half
                    .chars()
                    .map(|c| highlight_char(c, self.number, self.x, self.letter))
                    .collect::<String>();

                caps.name("second_half").map_or_else(
                    || format!("{formatted_prefix}{formatted_first_half}"),
                    |second_half| {
                        let formatted_second_half = second_half
                            .as_str()
                            .chars()
                            .map(|c| highlight_char(c, self.number, self.x, self.letter))
                            .collect::<String>();
                        format!(
                            "{}{}{}{}",
                            formatted_prefix,
                            formatted_first_half,
                            self.separator.paint(self.separator_token.to_string()),
                            formatted_second_half
                        )
                    },
                )
            })
            .to_string()
    }
}

fn highlight_char(c: char, number: Style, x: Style, letter: Style) -> String {
    match c {
        '0'..='9' => format!("{}", number.paint(c.to_string())),
        'x' | 'X' => format!("{}", x.paint(c.to_string())),
        'a'..='f' | 'A'..='F' => format!("{}", letter.paint(c.to_string())),
        _ => c.to_string(),
    }
}
