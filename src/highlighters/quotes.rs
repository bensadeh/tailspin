use crate::color;
use crate::color::to_ansi;
use crate::highlighters::quotes::State::{InsideQuote, OutsideQuote};
use crate::line_info::LineInfo;
use crate::theme::Style;
use crate::types::Highlight;

pub struct QuoteHighlighter {
    color: String,
    quotes_token: char,
}

impl QuoteHighlighter {
    pub fn new(style: &Style, quotes_token: char) -> Self {
        Self {
            color: to_ansi(style),
            quotes_token,
        }
    }
}

impl Highlight for QuoteHighlighter {
    fn apply(&self, input: &str, line_info: &LineInfo) -> String {
        self.highlight_inside_quotes(input, line_info)
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
    fn highlight_inside_quotes(&self, input: &str, line_info: &LineInfo) -> String {
        if line_info.double_quotes == 0 || line_info.double_quotes % 2 != 0 {
            return input.to_string();
        }

        let mut state = OutsideQuote;
        let mut output = String::new();

        for ch in input.chars() {
            match &mut state {
                InsideQuote {
                    color_inside_quote: color,
                    ref mut potential_reset_code,
                } => {
                    if ch == self.quotes_token {
                        output.push(ch);
                        output.push_str(color::RESET);
                        state = OutsideQuote;
                        continue;
                    }

                    potential_reset_code.push(ch);
                    if potential_reset_code.as_str() == color::RESET {
                        output.push_str(potential_reset_code);
                        output.push_str(color);
                        potential_reset_code.clear();
                    } else if !color::RESET.starts_with(potential_reset_code.as_str()) {
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
    use crate::color::Fg;

    #[test]
    fn highlight_quotes_with_ansi() {
        let style = Style {
            fg: Fg::Yellow,
            ..Default::default()
        };

        let line_info = LineInfo {
            double_quotes: 2,
            ..Default::default()
        };

        let highlighter = QuoteHighlighter::new(&style, '"');
        let result = highlighter.apply(
            "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside",
            &line_info,
        );
        let expected =
            "outside \x1b[33m\"hello \x1b[34;42;3m42\x1b[0m\x1b[33m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn highlight_quotes_without_ansi() {
        let style = Style {
            fg: Fg::Red,
            ..Default::default()
        };

        let line_info = LineInfo {
            double_quotes: 2,
            ..Default::default()
        };

        let highlighter = QuoteHighlighter::new(&style, '"');
        let input = "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        let result = highlighter.apply(input, &line_info);
        let expected =
            "outside \x1b[31m\"hello \x1b[34;42;3m42\x1b[0m\x1b[31m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn do_nothing_on_uneven_number_of_quotes() {
        let style = Style {
            fg: Fg::Red,
            ..Default::default()
        };

        let line_info = LineInfo {
            double_quotes: 1,
            ..Default::default()
        };

        let highlighter = QuoteHighlighter::new(&style, '"');
        let result = highlighter.apply(
            "outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside",
            &line_info,
        );
        let expected = "outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside";

        assert_eq!(result, expected);
    }
}
