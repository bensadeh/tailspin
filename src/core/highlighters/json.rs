use crate::core::config::JsonConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::Painter;
use serde_json::Value;
use std::borrow::Cow;
use std::fmt::Write;

pub struct JsonHighlighter {
    key: Painter,
    quote_token: Painter,
    curly_bracket: Painter,
    square_bracket: Painter,
    comma: Painter,
    colon: Painter,
}

impl JsonHighlighter {
    pub fn new(config: JsonConfig) -> Self {
        Self {
            key: Painter::new(config.key.into()),
            quote_token: Painter::new(config.quote_token.into()),
            curly_bracket: Painter::new(config.curly_bracket.into()),
            square_bracket: Painter::new(config.square_bracket.into()),
            comma: Painter::new(config.comma.into()),
            colon: Painter::new(config.colon.into()),
        }
    }

    fn format_json(&self, value: &Value, output: &mut String) {
        match value {
            Value::Object(map) => {
                self.curly_bracket.paint(output, "{");
                let mut first = true;
                for (key, val) in map {
                    if !first {
                        self.comma.paint(output, ",");
                    }
                    first = false;

                    output.push(' ');
                    self.quote_token.paint(output, "\"");
                    self.key.paint(output, key);
                    self.quote_token.paint(output, "\"");
                    self.colon.paint(output, ":");
                    output.push(' ');

                    self.format_json(val, output);
                }
                output.push(' ');
                self.curly_bracket.paint(output, "}");
            }
            Value::Array(array) => {
                self.square_bracket.paint(output, "[");
                let mut first = true;
                for item in array {
                    if !first {
                        self.comma.paint(output, ",");
                        output.push(' ');
                    }
                    first = false;

                    self.format_json(item, output);
                }
                self.square_bracket.paint(output, "]");
            }
            Value::String(s) => {
                self.quote_token.paint(output, "\"");
                output.push_str(s);
                self.quote_token.paint(output, "\"");
            }
            Value::Number(n) => {
                write!(output, "{n}").unwrap();
            }
            Value::Bool(b) => {
                write!(output, "{b}").unwrap();
            }
            Value::Null => {
                output.push_str("null");
            }
        }
    }
}

impl Highlight for JsonHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let first = input.as_bytes().iter().find(|b| !b.is_ascii_whitespace());
        if first != Some(&b'{') && first != Some(&b'[') {
            return Cow::Borrowed(input);
        }

        let json_value: Value = match serde_json::from_str(input) {
            Ok(value) => value,
            Err(_) => return Cow::Borrowed(input),
        };

        let mut output = String::with_capacity(input.len() * 2);
        self.format_json(&json_value, &mut output);

        Cow::Owned(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    #[test]
    fn test_number_highlighter() {
        let config = JsonConfig {
            key: Style::new().fg(Color::Yellow),
            quote_token: Style::new().fg(Color::Blue),
            curly_bracket: Style::new().fg(Color::Cyan),
            square_bracket: Style::new().fg(Color::Green),
            comma: Style::new().fg(Color::Red),
            colon: Style::new().fg(Color::Magenta),
        };
        let highlighter = JsonHighlighter::new(config);

        let cases = vec![
            (
                r#"{ "name": "John Doe", "age": 43, "phones": [ "+44 1234567", "+44 2345678" ] }"#,
                r#"[cyan]{[reset] [blue]"[reset][yellow]name[reset][blue]"[reset][magenta]:[reset] [blue]"[reset]John Doe[blue]"[reset][red],[reset] [blue]"[reset][yellow]age[reset][blue]"[reset][magenta]:[reset] 43[red],[reset] [blue]"[reset][yellow]phones[reset][blue]"[reset][magenta]:[reset] [green][[reset][blue]"[reset]+44 1234567[blue]"[reset][red],[reset] [blue]"[reset]+44 2345678[blue]"[reset][green]][reset] [cyan]}[reset]"#,
            ),
            (
                r#"{ "name": "John", "age": 30 }"#,
                r#"[cyan]{[reset] [blue]"[reset][yellow]name[reset][blue]"[reset][magenta]:[reset] [blue]"[reset]John[blue]"[reset][red],[reset] [blue]"[reset][yellow]age[reset][blue]"[reset][magenta]:[reset] 30 [cyan]}[reset]"#,
            ),
            ("No jsons here!", "No jsons here!"),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }
}
