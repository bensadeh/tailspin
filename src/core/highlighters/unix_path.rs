use crate::core::config::UnixPathConfig;
use crate::core::highlighter::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex, RegexBuilder};
use std::borrow::Cow;

pub struct UnixPathHighlighter {
    regex: Regex,
    segment: NuStyle,
    separator: NuStyle,
}

impl UnixPathHighlighter {
    pub fn new(config: UnixPathConfig) -> Result<Self, Error> {
        let pattern = r"(?x)
            (?P<path>
            (?:\./|~/|//|/)
                [\w.-]+
                (?:/[\w.-]+)+ |
            [\w.-]+ (?:/[\w.-]+){2,}
        ) ";
        let regex = RegexBuilder::new(pattern).unicode(false).build()?;

        Ok(Self {
            regex,
            segment: config.segment.into(),
            separator: config.separator.into(),
        })
    }

    fn paint_segment(&self, s: &str) -> String {
        self.segment.paint(s).to_string()
    }

    fn paint_separator(&self) -> String {
        self.separator.paint("/").to_string()
    }
}

impl Highlight for UnixPathHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if !input.as_bytes().contains(&b'/') {
            return Cow::Borrowed(input);
        }

        self.regex.replace_all(input, |caps: &Captures<'_>| {
            let path = &caps["path"];
            let mut out = String::new();
            let mut cur = String::new();

            for ch in path.chars() {
                if ch == '/' {
                    if !cur.is_empty() {
                        out.push_str(&self.paint_segment(&cur));
                        cur.clear();
                    }
                    out.push_str(&self.paint_separator());
                } else {
                    cur.push(ch);
                }
            }

            if !cur.is_empty() {
                out.push_str(&self.paint_segment(&cur));
            }

            out
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
            (
                "a/b/c",
                "[green]a[reset][yellow]/[reset][green]b[reset][yellow]/[reset][green]c[reset]",
            ),
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
            (
                "123/234/345/456",
                "[green]123[reset][yellow]/[reset][green]234[reset][yellow]/[reset][green]345[reset][yellow]/[reset][green]456[reset]",
            ),
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
