use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static PATH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)                   # Enable extended mode for readability
            (?P<path>            # Capture the path segment
                [~/.][\w./-]*    # Match zero or more word characters, dots, slashes, or hyphens
                /[\w.-]*         # Match a path segment separated by a slash
            )",
    )
    .expect("Invalid regex pattern")
});

pub struct PathHighlighter {
    segment: Style,
    separator: Style,
}

impl PathHighlighter {
    pub const fn new(segment: Style, separator: Style) -> Self {
        Self { segment, separator }
    }
}

impl Highlight for PathHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.slashes == 0
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        true
    }

    fn apply(&self, input: &str) -> String {
        PATH_REGEX
            .replace_all(input, |caps: &Captures<'_>| {
                let mut output = String::new();
                let path = &caps[0];
                let chars: Vec<_> = path.chars().collect();

                // Check if path starts with a valid character and not a double slash
                if !(chars[0] == '/' || chars[0] == '~' || (chars[0] == '.' && chars.len() > 1 && chars[1] == '/'))
                    || (chars[0] == '/' && chars.len() > 1 && chars[1] == '/')
                {
                    return path.to_string();
                }

                for &char in &chars {
                    match char {
                        '/' => output.push_str(&format!("{}", self.separator.paint(char.to_string()))),
                        _ => output.push_str(&format!("{}", self.segment.paint(char.to_string()))),
                    }
                }

                output
            })
            .to_string()
    }
}
