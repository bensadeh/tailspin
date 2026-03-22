use super::RegexExt;
use crate::core::config::EmailConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use memchr::memchr;
use regex::{Regex, RegexBuilder};
use std::borrow::Cow;

pub struct EmailHighlighter {
    regex: Regex,
    local_part: Painter,
    at_sign: Painter,
    domain: Painter,
    dot: Painter,
}

impl EmailHighlighter {
    pub fn new(config: EmailConfig) -> Self {
        let pattern = r"(?x)
            ([a-zA-Z0-9._%+-]+)          # local part
            (@)                           # at sign
            ([a-zA-Z0-9.-]+\.[a-zA-Z]{2,}) # domain
        ";

        let regex = RegexBuilder::new(pattern)
            .unicode(false)
            .build()
            .expect("hardcoded email regex must compile");

        Self {
            regex,
            local_part: Painter::new(config.local_part.into()),
            at_sign: Painter::new(config.at_sign.into()),
            domain: Painter::new(config.domain.into()),
            dot: Painter::new(config.dot.into()),
        }
    }
}

impl Highlight for EmailHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b'@', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            let local = caps.get(1).unwrap().as_str();
            let at = caps.get(2).unwrap().as_str();
            let domain = caps.get(3).unwrap().as_str();

            self.local_part.paint(buf, local);
            self.at_sign.paint(buf, at);

            let mut first = true;
            for segment in domain.split('.') {
                if !first {
                    self.dot.paint(buf, ".");
                }
                first = false;
                self.domain.paint(buf, segment);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    fn test_highlighter() -> EmailHighlighter {
        EmailHighlighter::new(EmailConfig {
            local_part: Style::new().fg(Color::Green),
            at_sign: Style::new().fg(Color::Red),
            domain: Style::new().fg(Color::Blue),
            dot: Style::new().fg(Color::Yellow),
        })
    }

    #[test]
    fn test_simple_email() {
        let highlighter = test_highlighter();
        let input = "Contact user@example.com for help";
        let expected =
            "Contact [green]user[reset][red]@[reset][blue]example[reset][yellow].[reset][blue]com[reset] for help";
        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_email_with_plus_and_dots() {
        let highlighter = test_highlighter();
        let input = "first.last+tag@sub.domain.co.uk";
        let expected = "[green]first.last+tag[reset][red]@[reset][blue]sub[reset][yellow].[reset][blue]domain[reset][yellow].[reset][blue]co[reset][yellow].[reset][blue]uk[reset]";
        let actual = highlighter.apply(input);
        assert_eq!(expected, actual.to_string().convert_escape_codes());
    }

    #[test]
    fn test_no_email() {
        let highlighter = test_highlighter();
        let input = "No email here!";
        let actual = highlighter.apply(input);
        assert!(matches!(actual, Cow::Borrowed(_)));
        assert_eq!("No email here!", actual.as_ref());
    }

    #[test]
    fn test_multiple_emails() {
        let highlighter = test_highlighter();
        let input = "From alice@a.com to bob@b.org";
        let actual = highlighter.apply(input);
        let result = actual.to_string().convert_escape_codes();
        assert!(result.contains("[green]alice[reset]"));
        assert!(result.contains("[green]bob[reset]"));
    }
}
