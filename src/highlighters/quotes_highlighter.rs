use crate::highlighters::quotes_highlighter::State::{InsideQuote, OutsideQuote};
use crate::highlighters::HighlightFn;

pub fn highlight_quotes(color: String) -> HighlightFn {
    Box::new(move |input: &str| -> String { highlight_string(&color, input) })
}

enum State {
    InsideQuote {
        color_inside_quote: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

fn highlight_string(color: &str, input: &str) -> String {
    const RESET: &str = "\x1b[0m";
    const QUOTE_SYMBOL: char = '"';

    let has_unmatched_quotes = input.chars().filter(|&ch| ch == QUOTE_SYMBOL).count() % 2 != 0;
    if has_unmatched_quotes {
        return input.to_string();
    }

    let mut state = OutsideQuote;
    let mut output = String::new();

    for ch in input.chars() {
        state = match (ch, &mut state) {
            (QUOTE_SYMBOL, InsideQuote { .. }) => {
                output.push_str(color);
                output.push(ch);
                output.push_str(RESET);
                OutsideQuote
            }
            (QUOTE_SYMBOL, OutsideQuote) => {
                output.push_str(color);
                output.push(ch);
                InsideQuote {
                    color_inside_quote: color.to_string(),
                    potential_reset_code: String::new(),
                }
            }
            (
                _,
                InsideQuote {
                    color_inside_quote: color,
                    ref mut potential_reset_code,
                },
            ) => {
                potential_reset_code.push(ch);
                if potential_reset_code.as_str() == RESET {
                    output.push_str(potential_reset_code);
                    output.push_str(color);
                    potential_reset_code.clear();
                } else if !RESET.starts_with(potential_reset_code.as_str()) {
                    output.push_str(potential_reset_code);
                    potential_reset_code.clear();
                }
                continue;
            }
            (_, OutsideQuote) => {
                output.push(ch);
                continue;
            }
        };
    }

    output
}
