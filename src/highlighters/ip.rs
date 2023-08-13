use crate::color;
use crate::color::to_ansi;
use crate::highlight_utils::highlight_with_awareness;
use crate::line_info::LineInfo;
use crate::regex::IP_ADDRESS_REGEX;
use crate::theme::Style;
use crate::types::Highlight;
use regex::{Captures, Regex};

pub struct IpHighlighter {
    segment_color: String,
    separator_color: String,
}

impl IpHighlighter {
    pub fn new(segment: &Style, separator: &Style) -> Self {
        let segment_color = to_ansi(segment);
        let separator_color = to_ansi(separator);
        IpHighlighter {
            segment_color,
            separator_color,
        }
    }
}

impl Highlight for IpHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        if line_info.dots < 3 {
            return true;
        }

        false
    }

    fn apply(&self, input: &str) -> String {
        highlight_ip_addresses(
            &self.segment_color,
            &self.separator_color,
            input,
            &IP_ADDRESS_REGEX,
        )
    }
}

fn highlight_ip_addresses(
    segment_color: &str,
    separator_color: &str,
    input: &str,
    ip_address_regex: &Regex,
) -> String {
    let highlight_groups = [
        (segment_color, 1),
        (separator_color, 2),
        (segment_color, 3),
        (separator_color, 4),
        (segment_color, 5),
        (separator_color, 6),
        (segment_color, 7),
    ];

    highlight_with_awareness(input, ip_address_regex, |caps: &Captures<'_>| {
        let mut output = String::new();
        for &(color, group) in &highlight_groups {
            output.push_str(&format!("{}{}{}", color, &caps[group], color::RESET));
        }
        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_ip_addresses() {
        let ip_address = "192.168.0.1";
        let segment_color = "\x1b[31m"; // ANSI color code for red
        let separator_color = "\x1b[32m"; // ANSI color code for green

        let highlighted = highlight_ip_addresses(
            segment_color,
            separator_color,
            ip_address,
            &IP_ADDRESS_REGEX,
        );

        let expected = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            segment_color,
            "192",
            color::RESET,
            separator_color,
            ".",
            color::RESET,
            segment_color,
            "168",
            color::RESET,
            separator_color,
            ".",
            color::RESET,
            segment_color,
            "0",
            color::RESET,
            separator_color,
            ".",
            color::RESET,
            segment_color,
            "1",
            color::RESET,
        );
        assert_eq!(highlighted, expected);
    }

    #[test]
    fn test_highlight_ip_addresses_no_ip() {
        let text = "this is a test string with no IP address";
        let segment_color = "\x1b[31m";
        let separator_color = "\x1b[32m";

        let highlighted =
            highlight_ip_addresses(segment_color, separator_color, text, &IP_ADDRESS_REGEX);

        // The input string does not contain an IP address, so it should be returned as-is
        assert_eq!(highlighted, text);
    }
}
