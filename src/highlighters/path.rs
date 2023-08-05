use crate::color;
use crate::color::to_ansi;
use crate::highlight_utils::highlight_with_awareness;
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::HighlightFn;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn highlight(segment: &Style, separator: &Style) -> HighlightFn {
    let segment_color = to_ansi(segment);
    let separator_color = to_ansi(separator);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_paths(
            &segment_color,
            &separator_color,
            input,
            line_info,
            &PATH_REGEX,
        )
    })
}

lazy_static! {
    static ref PATH_REGEX: Regex = {
        Regex::new(
            r"(?x)                        # Enable extended mode for readability
            (?P<path>                     # Capture the path segment
                [~/.][\w./-]*             # Match zero or more word characters, dots, slashes, or hyphens
                /[\w.-]*                  # Match a path segment separated by a slash
            )"
        ).expect("Invalid regex pattern")
    };
}

fn highlight_paths(
    segment_color: &str,
    separator_color: &str,
    input: &str,
    line_info: &LineInfo,
    path_regex: &Regex,
) -> String {
    if line_info.slashes == 0 {
        return input.to_string();
    }

    highlight_with_awareness(input, path_regex, |caps: &Captures<'_>| {
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

        for i in 0..chars.len() {
            if chars[i] == '/' {
                output.push_str(&format!("{}{}{}", separator_color, chars[i], color::RESET));
            } else {
                output.push_str(&format!("{}{}{}", segment_color, chars[i], color::RESET));
            }
        }
        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted =
            highlight_paths(segment_color, separator_color, path, line_info, &PATH_REGEX);

        let expected = path
            .chars()
            .map(|ch| {
                if ch == '/' {
                    format!("{}{}{}", separator_color, ch, color::RESET)
                } else {
                    format!("{}{}{}", segment_color, ch, color::RESET)
                }
            })
            .collect::<String>();
        assert_eq!(highlighted, expected);
    }

    #[test]
    fn test_highlight_paths_no_path() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 0,
            double_quotes: 0,
            colons: 0,
        };

        let text = "this is a test string with no path";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted =
            highlight_paths(segment_color, separator_color, text, line_info, &PATH_REGEX);

        // The input string does not contain a path, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
