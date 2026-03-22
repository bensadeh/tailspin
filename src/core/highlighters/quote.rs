use memchr::memchr_iter;
use nu_ansi_term::Style as NuStyle;
use std::borrow::Cow;

use crate::core::config::QuoteConfig;
use crate::core::highlighter::Highlight;
use crate::core::highlighters::quote::State::{InsideQuote, OutsideQuote};
use crate::style::Style;

const RESET: &str = "\x1b[0m";

pub struct QuoteHighlighter {
    quote_token: u8,
    color: String,
}

impl QuoteHighlighter {
    pub fn new(config: QuoteConfig) -> Self {
        let color = ansi_color_code_without_reset(config.style);

        Self {
            quote_token: config.quote_token,
            color,
        }
    }
}

fn ansi_color_code_without_reset(style: Style) -> String {
    let nu_style = NuStyle::from(style);
    let styled_str = format!("{}", nu_style.paint(""));

    styled_str.replace(RESET, "")
}

impl Highlight for QuoteHighlighter {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let quotes_count = memchr_iter(self.quote_token, input.as_bytes()).count();

        if quotes_count == 0 || !quotes_count.is_multiple_of(2) {
            return Cow::Borrowed(input);
        }

        let mut state = OutsideQuote;
        let mut output = String::with_capacity(input.len());

        for ch in input.chars() {
            match &mut state {
                InsideQuote { potential_reset_code } => {
                    if ch == self.quote_token as char {
                        // Flush any partially accumulated escape sequence.
                        output.push_str(potential_reset_code);
                        // End of a quoted segment: insert the closing quote and reset.
                        output.push(ch);
                        output.push_str(RESET);
                        state = OutsideQuote;
                        continue;
                    }

                    // Accumulate characters to see if we are matching a reset sequence.
                    potential_reset_code.push(ch);
                    if potential_reset_code.as_str() == RESET {
                        output.push_str(potential_reset_code);
                        output.push_str(&self.color);
                        potential_reset_code.clear();
                    } else if !RESET.starts_with(potential_reset_code.as_str()) {
                        // The accumulated characters do not form the reset code.
                        output.push_str(potential_reset_code);
                        potential_reset_code.clear();
                    }
                }
                OutsideQuote => {
                    if ch == self.quote_token as char {
                        // Start of a quoted segment: insert the color code and the quote.
                        output.push_str(&self.color);
                        output.push(ch);
                        state = InsideQuote {
                            potential_reset_code: String::with_capacity(RESET.len()),
                        };
                        continue;
                    }
                    output.push(ch);
                }
            }
        }

        Cow::Owned(output)
    }

    fn apply_to_line<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.apply(input)
    }
}

enum State {
    InsideQuote { potential_reset_code: String },
    OutsideQuote,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::{ConvertEscapeCodes, ConvertHighlightCodes};
    use crate::style::{Color, Style};

    #[test]
    fn test_multiple() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let cases = vec![
            (
                r#"Lorem ipsum "dolor" sit amet"#,
                r#"Lorem ipsum [yellow]"dolor"[reset] sit amet"#,
            ),
            (
                r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit"#,
                r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit"#,
            ),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.to_string().convert_escape_codes());
        }
    }

    #[test]
    fn test_no_overwrite() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#"Hello "abc [red]def[reset] ghi" World"#.to_string().convert_highlight_codes();
        let expected = r#"Hello [yellow]"abc [red]def[reset][yellow] ghi"[reset] World"#;

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_odd_number_of_highlight_tokens() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#"Hello "abc def ghi World"#;
        let expected = r#"Hello "abc def ghi World"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_preserves_multiple_highlights_inside_quotes() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#"Log "abc [red]error[reset] then [cyan]42[reset] end" done"#
            .to_string()
            .convert_highlight_codes();
        let expected = r#"Log [yellow]"abc [red]error[reset][yellow] then [cyan]42[reset][yellow] end"[reset] done"#;

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_highlight_at_start_of_quote() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#""[red]error[reset] occurred""#.to_string().convert_highlight_codes();
        let expected = r#"[yellow]"[red]error[reset][yellow] occurred"[reset]"#;

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_highlight_at_end_of_quote() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#""something [red]error[reset]""#.to_string().convert_highlight_codes();
        let expected = r#"[yellow]"something [red]error[reset][yellow]"[reset]"#;

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_no_highlights_inside_quotes() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#"Hello "plain text" world"#;
        let expected = r#"Hello [yellow]"plain text"[reset] world"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_adjacent_quoted_strings() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#""hello""world""#;
        let expected = r#"[yellow]"hello"[reset][yellow]"world"[reset]"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_empty_quoted_string() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#"before "" after"#;
        let expected = r#"before [yellow]""[reset] after"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_entirely_highlighted_content_inside_quotes() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'"',
            style: Style::new().fg(Color::Yellow),
        });

        let input = r#""[red]error[reset]""#.to_string().convert_highlight_codes();
        let expected = r#"[yellow]"[red]error[reset][yellow]"[reset]"#;

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_single_quote_token() {
        let highlighter = QuoteHighlighter::new(QuoteConfig {
            quote_token: b'\'',
            style: Style::new().fg(Color::Yellow),
        });

        let input = "msg 'hello [red]world[reset] end' done"
            .to_string()
            .convert_highlight_codes();
        let expected = "msg [yellow]'hello [red]world[reset][yellow] end'[reset] done";

        let actual = highlighter.apply(input.as_str());

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }
}
