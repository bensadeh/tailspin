use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use regex::{Captures, Regex};

pub struct KeywordHighlighter {
    regex: Regex,
    style: Style,
    border: bool,
}

impl KeywordHighlighter {
    pub fn new(keywords: &[String], style: Style, border: bool) -> Self {
        let keyword_pattern = keywords
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>()
            .join("|");

        let regex = Regex::new(&format!(r"\b({keyword_pattern})\b")).expect("Invalid regex pattern");

        Self { regex, style, border }
    }
}

impl Highlight for KeywordHighlighter {
    fn should_short_circuit(&self, _line_info: &LineInfo) -> bool {
        false
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
                if self.border {
                    format!(
                        "{}{}{}",
                        self.style.paint(" "),
                        self.style.paint(&caps[0]),
                        self.style.paint(" ")
                    )
                } else {
                    format!("{}", self.style.paint(&caps[0]))
                }
            })
            .to_string()
    }
}
