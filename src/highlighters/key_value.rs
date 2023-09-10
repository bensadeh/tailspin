use crate::color::to_ansi;
use crate::line_info::LineInfo;
use crate::regex::KEY_VALUE_REGEX;
use crate::theme::Style;
use crate::types::Highlight;

pub struct KeyValueHighlighter {
    key_color: String,
    equals_sign_color: String,
}

impl KeyValueHighlighter {
    pub fn new(key_style: &Style, equals_sign_style: &Style) -> Self {
        Self {
            key_color: to_ansi(key_style),
            equals_sign_color: to_ansi(equals_sign_style),
        }
    }
}

impl Highlight for KeyValueHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.equals < 1 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_key_values(&self.key_color, &self.equals_sign_color, input)
    }
}

fn highlight_key_values(key_color: &str, equals_sign_color: &str, input: &str) -> String {
    KEY_VALUE_REGEX
        .replace_all(input, |captures: &regex::Captures| {
            let space_or_start = captures.name("space_or_start").map(|s| s.as_str()).unwrap_or_default();
            let key = captures
                .name("key")
                .map(|k| format!("{}{}\x1B[0m", key_color, k.as_str()))
                .unwrap_or_default();
            let equals_sign = captures
                .name("equals")
                .map(|e| format!("{}{}\x1B[0m", equals_sign_color, e.as_str()))
                .unwrap_or_default();

            format!("{}{}{}", space_or_start, key, equals_sign)
        })
        .to_string()
}
