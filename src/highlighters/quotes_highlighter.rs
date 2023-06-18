use crate::highlighters::quotes_highlighter::State::{InsideQuote, OutsideQuote};
use crate::highlighters::HighlightFn;

pub fn highlight_quotes(color: String, quote_symbol: char) -> HighlightFn {
    Box::new(move |input: &str| -> String { highlight_string(&color, input, quote_symbol) })
}

enum State {
    InsideQuote {
        color_inside_quote: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

fn highlight_string(color: &str, input: &str, quote_symbol: char) -> String {
    const RESET: &str = "\x1b[0m";

    let has_unmatched_quotes = input.chars().filter(|&ch| ch == quote_symbol).count() % 2 != 0;
    if has_unmatched_quotes {
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
                if ch == quote_symbol {
                    output.push(ch);
                    output.push_str(RESET);
                    state = OutsideQuote;
                } else {
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
            }
            OutsideQuote => {
                if ch == quote_symbol {
                    output.push_str(color);
                    output.push(ch);
                    state = InsideQuote {
                        color_inside_quote: color.to_string(),
                        potential_reset_code: String::new(),
                    };
                } else {
                    output.push(ch);
                }
            }
        };
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_quotes_with_ansi() {
        let ansi_red = String::from("\x1b[33");
        let highlighter = highlight_quotes(ansi_red, '"');
        let result = highlighter("outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected =
            "outside \x1b[33\"hello \x1b[34;42;3m42\x1b[0m\x1b[33 world\"\x1b[0m outside";
        assert_eq!(result, expected);
    }

    #[test]
    fn highlight_quotes_without_ansi() {
        let color = String::from("[color]");
        let highlighter = highlight_quotes(color, '"');
        let result = highlighter("outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected =
            "outside [color]\"hello \x1b[34;42;3m42\x1b[0m[color] world\"\x1b[0m outside";
        assert_eq!(result, expected);
    }

    #[test]
    fn do_nothing_on_uneven_number_of_quotes() {
        let color = String::from("[color]");
        let highlighter = highlight_quotes(color, '"');
        let result = highlighter("outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside");
        let expected = "outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        assert_eq!(result, expected);
    }
}
