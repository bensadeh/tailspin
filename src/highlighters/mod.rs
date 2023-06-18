mod numbers;
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

        before_fns.push(numbers::highlight(color_for_numbers.to_string()));
        after_fns.push(quotes::highlight(color_for_quotes.to_string(), '"'));

        Highlighters {
            before: before_fns,
            after: after_fns,
        }
    }
}
