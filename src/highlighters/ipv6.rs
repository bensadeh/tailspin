use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static IPV6_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
            \b(?:                       # Start of non-capturing group
                ([A-Fa-f0-9]{1,4}:){1,7}[A-Fa-f0-9]{1,4}    # Standard IPv6 (1-8 groups)
                |                                             # OR
                ::([A-Fa-f0-9]{1,4}:){1,6}[A-Fa-f0-9]{1,4}   # Leading :: (1-7 groups)
                |                                             # OR
                ([A-Fa-f0-9]{1,4}:){1,6}::                    # Trailing :: (1-7 groups)
                |                                             # OR
                ([A-Fa-f0-9]{1,4}:){1,5}(:[A-Fa-f0-9]{1,4}){1,2} # Middle :: (1-6 groups)
                |                                             # OR
                ([A-Fa-f0-9]{1,4}:){1,4}(:[A-Fa-f0-9]{1,4}){1,3} # Middle :: (1-5 groups)
                |                                             # OR
                ([A-Fa-f0-9]{1,4}:){1,3}(:[A-Fa-f0-9]{1,4}){1,4} # Middle :: (1-4 groups)
                |                                             # OR
                ([A-Fa-f0-9]{1,4}:){1,2}(:[A-Fa-f0-9]{1,4}){1,5} # Middle :: (1-3 groups)
                |                                             # OR
                ([A-Fa-f0-9]{1,4}:){1}(:[A-Fa-f0-9]{1,4}){1,6}   # Middle :: (1-2 groups)
                |                                             # OR
                ::                                            # Just ::
            )\b",
    )
    .expect("Invalid IPv6 regex pattern")
});

pub struct Ipv6Highlighter {
    number: Style,
    letter: Style,
    separator: Style,
}

impl Ipv6Highlighter {
    pub fn new(number: Style, letter: Style, separator: Style) -> Self {
        Self {
            number,
            letter,
            separator,
        }
    }
}

impl Highlight for Ipv6Highlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.colons < 4
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        IPV6_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                caps[0]
                    .chars()
                    .map(|c| match c {
                        '0'..='9' => self.number.paint(c.to_string()).to_string(),
                        'a'..='f' | 'A'..='F' => self.letter.paint(c.to_string()).to_string(),
                        ':' => self.separator.paint(c.to_string()).to_string(),
                        _ => c.to_string(),
                    })
                    .collect::<String>()
            })
            .to_string()
    }
}
