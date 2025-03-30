use crate::core::config::UnixPathConfig;
use crate::core::core::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct UnixPathHighlighter {
    regex: Regex,
    segment: NuStyle,
    separator: NuStyle,
}

impl UnixPathHighlighter {
    pub fn new(config: UnixPathConfig) -> Result<Self, Error> {
        let regex = Regex::new(
            r"(?x)               # Enable comments and whitespace insensitivity
            (?P<path>            # Capture the path segment
                [~/.][\w./-]*    # Match zero or more word characters, dots, slashes, or hyphens
                /[\w.-]*         # Match a path segment separated by a slash
            )",
        )?;

        Ok(Self {
            regex,
            segment: config.segment.into(),
            separator: config.separator.into(),
        })
    }
}

impl Highlight for UnixPathHighlighter {
    fn apply(&self, input: &str) -> String {
        self.regex
            .replace_all(input, |caps: &Captures<'_>| {
                let path = &caps["path"];
                let chars: Vec<_> = path.chars().collect();

                // Check if path starts with a valid character and not a double slash
                if !(chars[0] == '/' || chars[0] == '~' || (chars[0] == '.' && chars.len() > 1 && chars[1] == '/'))
                    || (chars[0] == '/' && chars.len() > 1 && chars[1] == '/')
                {
                    return path.to_string();
                }

                let mut output = String::new();
                let mut current_segment = String::new();
                for &char in &chars {
                    match char {
                        '/' => {
                            if !current_segment.is_empty() {
                                output.push_str(&self.segment.paint(&current_segment).to_string());
                                current_segment.clear();
                            }
                            output.push_str(&self.separator.paint(char.to_string()).to_string());
                        }
                        _ => current_segment.push(char),
                    }
                }

                if !current_segment.is_empty() {
                    output.push_str(&self.segment.paint(&current_segment).to_string());
                }

                output
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::{Color, Style};

    #[test]
    fn test_unix_path_highlighter() {
        let highlighter = UnixPathHighlighter::new(UnixPathConfig {
            segment: Style::new().fg(Color::Green),
            separator: Style::new().fg(Color::Yellow),
        })
        .unwrap();

        let cases = vec![
            (
                "/user/local",
                "[yellow]/[reset][green]user[reset][yellow]/[reset][green]local[reset]",
            ),
            ("No numbers here!", "No numbers here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
