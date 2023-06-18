use crate::color::{Fg, RESET};
use crate::config_parser::{Config, Settings};
use crate::config_util::FlattenKeyword;
use crate::highlighters::State::{InsideQuote, OutsideQuote};
use regex::Regex;

type HighlightFn = Box<dyn Fn(&str) -> String + Send>;
type HighlightFnVec = Vec<HighlightFn>;

pub struct Highlighters {
    pub before: HighlightFnVec,
    pub after: HighlightFnVec,
}

enum State {
    InsideQuote {
        color: String,
        potential_reset_code: String,
    },
    OutsideQuote,
}

impl Highlighters {
    pub fn new(config: Config, keywords: Vec<FlattenKeyword>) -> Highlighters {
        let mut before_fns: HighlightFnVec = Vec::new();
        let mut after_fns: HighlightFnVec = Vec::new();

        let color_for_numbers = Fg::Blue;
        let color_for_quotes = Fg::Yellow;

        before_fns.push(Highlighters::highlight_numbers(color_for_numbers));
        after_fns.push(Highlighters::highlight_quotes(color_for_quotes.to_string()));

        Highlighters {
            before: before_fns,
            after: after_fns,
        }
    }

    fn highlight_numbers(color: Fg) -> HighlightFn {
        Box::new(move |s: &str| -> String {
            let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

            let highlighted = number_regex.replace_all(s, |caps: &regex::Captures<'_>| {
                format!("{}{}{}", color, &caps[0], RESET)
            });

            highlighted.into_owned()
        })
    }

    fn highlight_quotes(color: String) -> HighlightFn {
        const RESET: &str = "\x1b[0m";
        const QUOTE_SYMBOL: char = '"';

        Box::new(move |input: &str| -> String {
            let has_unmatched_quotes =
                input.chars().filter(|&ch| ch == QUOTE_SYMBOL).count() % 2 != 0;
            if has_unmatched_quotes {
                return input.to_string();
            }

            let mut state = OutsideQuote;
            let mut output = String::new();

            for ch in input.chars() {
                state = match (ch, &mut state) {
                    (QUOTE_SYMBOL, InsideQuote { .. }) => {
                        output.push_str(&color);
                        output.push(ch);
                        output.push_str(RESET);
                        OutsideQuote
                    }
                    (QUOTE_SYMBOL, OutsideQuote) => {
                        output.push_str(&color);
                        output.push(ch);
                        InsideQuote {
                            color: color.clone(),
                            potential_reset_code: String::new(),
                        }
                    }
                    (
                        _,
                        InsideQuote {
                            color,
                            ref mut potential_reset_code,
                        },
                    ) => {
                        potential_reset_code.push(ch);
                        if potential_reset_code.as_str() == RESET {
                            output.push_str(potential_reset_code);
                            output.push_str(&color);
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
        })
    }
}
