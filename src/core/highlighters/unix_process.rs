use super::RegexExt;
use crate::core::config::UnixProcessConfig;
use crate::core::highlighter::Highlight;
use memchr::memchr;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct UnixProcessHighlighter {
    regex: Regex,
    name: NuStyle,
    id: NuStyle,
    bracket: NuStyle,
}

impl UnixProcessHighlighter {
    pub fn new(config: UnixProcessConfig) -> Result<Self, Error> {
        let pattern = r"(?P<process_name>\([A-Za-z0-9._ +:/-]+\)|[A-Za-z0-9_/-]+)\[(?P<process_id>\d+)]";
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            name: config.name.into(),
            id: config.id.into(),
            bracket: config.bracket.into(),
        })
    }
}

impl Highlight for UnixProcessHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b'[', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            if let Some(p) = caps.name("process_name") {
                let _ = write!(buf, "{}", self.name.paint(p.as_str()));
            }
            let _ = write!(buf, "{}", self.bracket.paint("["));
            if let Some(n) = caps.name("process_id") {
                let _ = write!(buf, "{}", self.id.paint(n.as_str()));
            }
            let _ = write!(buf, "{}", self.bracket.paint("]"));
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_unix_process_highlighter() {
        let highlighter = UnixProcessHighlighter::new(UnixProcessConfig {
            name: Style::new().fg(Color::Magenta),
            id: Style::new().fg(Color::Green),
            bracket: Style::new().fg(Color::Blue),
        })
        .unwrap();

        let cases = vec![
            (
                "process[1]",
                "[magenta]process[reset][blue][[reset][green]1[reset][blue]][reset]",
            ),
            (
                "postfix/postscreen[1894]: CONNECT from [192.168.1.22]:12345 to [127.0.0.1]:25",
                "[magenta]postfix/postscreen[reset][blue][[reset][green]1894[reset][blue]][reset]: CONNECT from [192.168.1.22]:12345 to [127.0.0.1]:25",
            ),
            ("No process here!", "No process here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
