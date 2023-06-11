use crate::color::{Fg, RESET};
use crate::config_parser::{Config, Settings};
use crate::config_util::FlattenKeyword;
use regex::Regex;

type HighlightFn = Box<dyn Fn(&str) -> String + Send>;
type HighlightFnVec = Vec<HighlightFn>;

pub struct Highlighters {
    pub before: HighlightFnVec,
    pub after: HighlightFnVec,
}

impl Highlighters {
    pub fn new(config: Config, keywords: Vec<FlattenKeyword>) -> Highlighters {
        let mut before_fns: HighlightFnVec = Vec::new();
        let mut after_fns: HighlightFnVec = Vec::new();

        let color_for_numbers = Fg::Green;
        let color_for_quotes = Fg::Magenta;

        before_fns.push(Highlighters::highlight_numbers(color_for_numbers));
        after_fns.push(Highlighters::highlight_quotes(color_for_quotes));

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

    fn highlight_quotes(color: Fg) -> HighlightFn {
        Box::new(move |input: &str| -> String {
            // Implement your highlighting logic here with respect to color
            // The color value is captured by the closure and can be used here
            let quote_count: usize = input.chars().filter(|&ch| ch == '"').count();
            if quote_count % 2 != 0 {
                return input.to_string();
            }

            let mut output = String::new();
            let mut inside_quote = false;
            let mut potential_color_code = String::new();

            for ch in input.chars() {
                if ch == '"' {
                    inside_quote = !inside_quote;
                    if inside_quote {
                        output.push_str(&color.to_string());
                        output.push(ch);
                    } else {
                        output.push(ch);
                        output.push_str(RESET);
                    }
                    continue;
                }

                if inside_quote {
                    potential_color_code.push(ch);

                    if potential_color_code == RESET {
                        output.push_str(&potential_color_code);
                        output.push_str(&color.to_string());
                        potential_color_code.clear();
                    } else if !RESET.starts_with(&potential_color_code) {
                        output.push_str(&potential_color_code);
                        potential_color_code.clear();
                    }
                } else {
                    output.push(ch);
                }
            }

            output
        })
    }
}
