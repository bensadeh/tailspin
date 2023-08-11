use crate::color;
use crate::color::to_ansi;
use crate::highlight_utils::highlight_with_awareness;
use crate::line_info::LineInfo;
use crate::regexes::PATH_REGEX;
use crate::theme::Style;
use crate::types::Highlight;
use regex::Captures;

pub struct PathHighlighter {
    segment_color: String,
    separator_color: String,
}

impl PathHighlighter {
    pub fn new(segment: &Style, separator: &Style) -> Self {
        Self {
            segment_color: to_ansi(segment),
            separator_color: to_ansi(separator),
        }
    }

    fn highlight_paths(&self, input: &str, line_info: &LineInfo) -> String {
        if line_info.slashes == 0 {
            return input.to_string();
        }

        highlight_with_awareness(input, &PATH_REGEX, |caps: &Captures<'_>| {
            let mut output = String::new();
            let path = &caps[0];
            let chars: Vec<_> = path.chars().collect();

            // Check if path starts with a valid character and not a double slash
            if !(chars[0] == '/'
                || chars[0] == '~'
                || (chars[0] == '.' && chars.len() > 1 && chars[1] == '/'))
                || (chars[0] == '/' && chars.len() > 1 && chars[1] == '/')
            {
                return path.to_string();
            }

            for &char in &chars {
                match char {
                    '/' => output.push_str(&format!(
                        "{}{}{}",
                        &self.separator_color,
                        char,
                        color::RESET
                    )),
                    _ => {
                        output.push_str(&format!("{}{}{}", &self.segment_color, char, color::RESET))
                    }
                }
            }

            output
        })
    }
}

impl Highlight for PathHighlighter {
    fn apply(&self, input: &str, line_info: &LineInfo) -> String {
        self.highlight_paths(input, line_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Fg;

    #[test]
    fn test_highlight_paths() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 1,
            double_quotes: 0,
            colons: 0,
        };

        let path = "~/Documents/../user/.";
        let segment_style = Style {
            fg: Fg::Red,
            ..Default::default()
        };
        let separator_style = Style {
            fg: Fg::Green,
            ..Default::default()
        };

        let highlighter = PathHighlighter::new(&segment_style, &separator_style);
        let highlighted = highlighter.apply(path, &line_info);

        let expected = path
            .chars()
            .map(|ch| {
                if ch == '/' {
                    format!("{}{}{}", to_ansi(&separator_style), ch, color::RESET)
                } else {
                    format!("{}{}{}", to_ansi(&segment_style), ch, color::RESET)
                }
            })
            .collect::<String>();
        assert_eq!(highlighted, expected);
    }

    #[test]
    fn test_highlight_paths_no_path() {
        let line_info = LineInfo::default();

        let text = "this is a test string with no path";
        let segment_style = Style {
            fg: Fg::Red,
            ..Default::default()
        };
        let separator_style = Style {
            fg: Fg::Green,
            ..Default::default()
        };

        let highlighter = PathHighlighter::new(&segment_style, &separator_style);
        let highlighted = highlighter.apply(text, &line_info);

        // The input string does not contain a path, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
