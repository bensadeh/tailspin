use super::RegexExt;
use crate::core::config::UnixPathConfig;
use crate::core::highlighter::Highlight;
use memchr::memchr;
use nu_ansi_term::Style as NuStyle;
use regex::{Error, Regex, RegexBuilder};
use std::borrow::Cow;
use std::fmt::Write as _;

pub struct UnixPathHighlighter {
    regex: Regex,
    segment: NuStyle,
    separator: NuStyle,
}

impl UnixPathHighlighter {
    pub fn new(config: UnixPathConfig) -> Result<Self, Error> {
        let pattern = r"(?x)
            (?:^|\s)
            (?P<path>
                (?:\./|~/|//|/)
                [\w.-]+
                (?:/[\w.-]+)+
            )
        ";
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            segment: config.segment.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for UnixPathHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if memchr(b'/', input.as_bytes()).is_none() {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all_cow(input, |caps, buf| {
            let full = caps.get(0).unwrap().as_str();
            let path = &caps["path"];

            // Preserve any leading whitespace that was part of the match
            if full.len() > path.len() {
                buf.push_str(&full[..full.len() - path.len()]);
            }

            let mut seg_start = None;

            for (i, ch) in path.char_indices() {
                if ch == '/' {
                    if let Some(start) = seg_start.take() {
                        let _ = write!(buf, "{}", self.segment.paint(&path[start..i]));
                    }
                    let _ = write!(buf, "{}", self.separator.paint("/"));
                } else if seg_start.is_none() {
                    seg_start = Some(i);
                }
            }

            if let Some(start) = seg_start {
                let _ = write!(buf, "{}", self.segment.paint(&path[start..]));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_unix_path_highlighter() {
        let highlighter = UnixPathHighlighter::new(UnixPathConfig {
            segment: Style::new().fg(Color::Green),
            separator: Style::new().fg(Color::Yellow),
        })
        .unwrap();

        let cases = vec![
            ("a//b", "a//b"),
            ("a/b", "a/b"),
            ("name/name", "name/name"),
            ("a/b/c", "a/b/c"),
            ("justtext", "justtext"),
            ("README.md", "README.md"),
            (
                "//network/share",
                "[yellow]/[reset][yellow]/[reset][green]network[reset][yellow]/[reset][green]share[reset]",
            ),
            (
                "/user/local",
                "[yellow]/[reset][green]user[reset][yellow]/[reset][green]local[reset]",
            ),
            ("123/234/345/456", "123/234/345/456"),
            (
                "~/projects/rust/tailspin",
                "[green]~[reset][yellow]/[reset][green]projects[reset][yellow]/[reset][green]rust[reset][yellow]/[reset][green]tailspin[reset]",
            ),
            (
                "./a/b",
                "[green].[reset][yellow]/[reset][green]a[reset][yellow]/[reset][green]b[reset]",
            ),
            (
                "/var/log/nginx/error.log",
                "[yellow]/[reset][green]var[reset][yellow]/[reset][green]log[reset][yellow]/[reset][green]nginx[reset][yellow]/[reset][green]error.log[reset]",
            ),
            (
                "/path/.hidden/file",
                "[yellow]/[reset][green]path[reset][yellow]/[reset][green].hidden[reset][yellow]/[reset][green]file[reset]",
            ),
            (
                "/usr/local/",
                "[yellow]/[reset][green]usr[reset][yellow]/[reset][green]local[reset]/",
            ),
            (
                "See /etc/hosts please",
                "See [yellow]/[reset][green]etc[reset][yellow]/[reset][green]hosts[reset] please",
            ),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
