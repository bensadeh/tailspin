use crate::color::to_ansi;
use crate::highlight_utils;
use crate::line_info::LineInfo;
use crate::regex::DATE_REGEX;
use crate::theme::{Shorten, Style};
use crate::types::Highlight;

pub struct DateHighlighter {
    style: String,
    shorten: Option<Shorten>,
}

impl DateHighlighter {
    pub fn new(style: &Style, shorten: Option<Shorten>) -> Self {
        Self {
            style: to_ansi(style),
            shorten,
        }
    }
}

impl Highlight for DateHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.dashes < 2 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        if let Some(shorten) = &self.shorten {
            return highlight_utils::replace_with_awareness(
                to_ansi(&shorten.clone().style).as_str(),
                input,
                &shorten.to,
                &DATE_REGEX,
            );
        }

        highlight_utils::highlight_with_awareness_replace_all(&self.style, input, &DATE_REGEX, false)
    }
}
