use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlight_utils::highlight_with_awareness;
use crate::highlighters::HighlightFn;
use regex::{Captures, Regex};

pub fn highlight(segment: &Style, separator: &Style) -> HighlightFn {
    let segment_color = to_ansi(segment);
    let separator_color = to_ansi(separator);

    Box::new(move |input: &str| -> String {
        highlight_paths(&segment_color, &separator_color, input)
    })
}

fn highlight_paths(segment_color: &str, separator_color: &str, input: &str) -> String {
    let path_regex = Regex::new(
        r"(?x)                    # Enable extended mode for readability
    (?P<leading>^|[\s(])      # Capture the leading boundary (start of string or whitespace/parenthesis)
    (?P<path>                 # Capture the path segment
        [~/.]                 # Match a special character (~ or .)
        [\w.-]*               # Match zero or more word characters, dots, or hyphens
        (/[^\s/][\w.-]*)*     # Match zero or more path segments separated by slashes
    )"
    ).expect("Invalid regex pattern");

    highlight_with_awareness(input, &path_regex, |caps: &Captures<'_>| {
        let mut output = String::new();
        let chars = caps[2].chars().peekable();
        for ch in chars {
            if ch == '/' {
                output.push_str(&format!("{}{}{}", separator_color, ch, color::RESET));
            } else {
                output.push_str(&format!("{}{}{}", segment_color, ch, color::RESET));
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
        let path = "~/Documents/../user/.";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted = highlight_paths(segment_color, separator_color, path);

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
        let text = "this is a test string with no path";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted = highlight_paths(segment_color, separator_color, text);

        // The input string does not contain a path, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
