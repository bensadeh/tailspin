use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlighters::quotes::State::{InsideQuote, OutsideQuote};
use crate::highlighters::HighlightFn;
use crate::line_info::LineInfo;

pub fn highlight(style: &Style, quotes_token: char) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str, line_info: &LineInfo| -> String {
        highlight_inside_quotes(&color, input, quotes_token, line_info)
    })
}

enum State {
    InsideQuote {
        color_inside_quote: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

fn highlight_inside_quotes(
    color: &str,
    input: &str,
    quotes_token: char,
    line_info: &LineInfo,
) -> String {
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
                if ch == quotes_token {
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
                if ch == quotes_token {
                    output.push_str(color);
                    output.push(ch);
                    state = InsideQuote {
                        color_inside_quote: color.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Bg, Fg};

    #[test]
    fn highlight_quotes_with_ansi() {
        let style = Style {
            fg: Fg::Yellow,
            bg: Bg::None,
            italic: false,
            bold: false,
            underline: false,
            faint: false,
        };

        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 0,
            double_quotes: 2,
            colons: 0,
        };

        let highlighter = highlight(&style, '"');
        let result = highlighter(
            "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside",
            line_info,
        );
        let expected =
            "outside \x1b[33m\"hello \x1b[34;42;3m42\x1b[0m\x1b[33m world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn highlight_quotes_without_ansi() {
        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 0,
            double_quotes: 2,
            colons: 0,
        };
        let color = "[color]";
        let input = "outside \"hello \x1b[34;42;3m42\x1b[0m world\" outside";
        let result = highlight_inside_quotes(color, input, '"', line_info);
        let expected =
            "outside [color]\"hello \x1b[34;42;3m42\x1b[0m[color] world\"\x1b[0m outside";

        assert_eq!(result, expected);
    }

    #[test]
    fn do_nothing_on_uneven_number_of_quotes() {
        let style = Style {
            fg: Fg::Red,
            bg: Bg::None,
            italic: false,
            bold: false,
            underline: false,
            faint: false,
        };

        let line_info = &LineInfo {
            dashes: 0,
            dots: 0,
            slashes: 0,
            double_quotes: 1,
            colons: 0,
        };

        let highlighter = highlight(&style, '"');
        let result = highlighter(
            "outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside",
            line_info,
        );
        let expected = "outside \" \"hello \x1b[34;42;3m42\x1b[0m world\" outside";

        assert_eq!(result, expected);
    }
}
