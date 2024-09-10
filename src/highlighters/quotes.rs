use crate::highlighters::quotes::State::{InsideQuote, OutsideQuote};
use crate::line_info::LineInfo;
use crate::types::Highlight;
use nu_ansi_term::Style;

const RESET: &str = "\x1b[0m";

pub struct QuoteHighlighter {
    color: String,
    quotes_token: char,
}

impl QuoteHighlighter {
    pub fn new(style: Style, quotes_token: char) -> Self {
        Self {
            color: extract_ansi_codes_without_reset(style),
            quotes_token,
        }
    }
}

fn extract_ansi_codes_without_reset(style: Style) -> String {
    let styled_str = format!("{}", style.paint(""));

    styled_str.replace("\x1b[0m", "")
}

impl Highlight for QuoteHighlighter {
    fn should_short_circuit(&self, line_info: &LineInfo) -> bool {
        line_info.double_quotes == 0 || line_info.double_quotes % 2 != 0
    }

    fn only_apply_to_segments_not_already_highlighted(&self) -> bool {
        false
    }

    fn apply(&self, input: &str) -> String {
        self.highlight_inside_quotes(input)
    }
}

enum State {
    InsideQuote {
        color_inside_quote: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

impl QuoteHighlighter {
    fn highlight_inside_quotes(&self, input: &str) -> String {
        let mut state = OutsideQuote;
        let mut output = String::with_capacity(input.len() + 16);

        for ch in input.chars() {
            match &mut state {
                InsideQuote {
                    color_inside_quote: color,
                    ref mut potential_reset_code,
                } => {
                    if ch == self.quotes_token {
                        output.push(ch);
                        output.push_str(RESET);
                        state = OutsideQuote;
                        continue;
                    }

                    potential_reset_code.push(ch);
                    if potential_reset_code.as_str() == RESET {
                        output.push_str(potential_reset_code);
                        output.push_str(color);
                        potential_reset_code.clear();
                    } else if !RESET.starts_with(potential_reset_code.as_str()) {
                        output.push_str(potential_reset_code);
                        potential_reset_code.clear();
                    }
                }
                OutsideQuote => {
                    if ch == self.quotes_token {
                        output.push_str(&self.color);
                        output.push(ch);
                        state = InsideQuote {
                            color_inside_quote: self.color.clone(),
                            potential_reset_code: String::new(),
                        };
                        continue;
                    }

                    output.push(ch);
                }
            };
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nu_ansi_term::Color;

    #[test]
    fn highlight_quotes_with_ansi() {
        let highlighter = QuoteHighlighter::new(Style::from(Color::Yellow), '"');

        let result = highlighter.apply("outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected = "outside \x1b[33m\"hello \x1b[34;42;3m42\x1b[0m\x1b[33m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn highlight_quotes_without_ansi() {
        let highlighter = QuoteHighlighter::new(Style::from(Color::Red), '"');

        let input = "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        let result = highlighter.apply(input);
        let expected = "outside \x1b[31m\"hello \x1b[34;42;3m42\x1b[0m\x1b[31m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn do_nothing_on_uneven_number_of_quotes() {
        let line_info = LineInfo {
            double_quotes: 1,
            ..Default::default()
        };

        let highlighter = QuoteHighlighter::new(Style::from(Color::Yellow), '"');
        let should_short_circuit_actual = highlighter.should_short_circuit(&line_info);

        assert!(should_short_circuit_actual);
    }
}
