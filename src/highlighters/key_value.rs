use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

static KEY_VALUE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<space_or_start>(^)|\s)(?P<key>\w+\b)(?P<equals>=)").expect("Invalid regex pattern"));

pub struct KeyValueHighlighter {
    key: Style,
    separator: Style,
}

impl KeyValueHighlighter {
    pub const fn new(key: Style, separator: Style) -> Self {
        Self { key, separator }
    }
}

impl Highlight for KeyValueHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.equals < 1
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        KEY_VALUE_REGEX
            .replace_all(input, |captures: &regex::Captures| {
                let space_or_start = &captures["space_or_start"];
                let key = &captures["key"];
                let equals_sign = &captures["equals"];

                format!(
                    "{space_or_start}{}{}",
                    self.key.paint(key),
                    self.separator.paint(equals_sign)
                )
            })
            .to_string()
    }
}
