mod quotes;

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

        let color_for_numbers = Fg::Blue;
        let color_for_quotes = Fg::Yellow;

        before_fns.push(Highlighters::highlight_numbers(color_for_numbers));
        after_fns.push(quotes::highlight(color_for_quotes.to_string(), '"'));

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
}
