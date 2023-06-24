mod keyword;
mod numbers;
mod quotes;

use crate::color::{Fg, RESET};
use crate::config_parser::{Config, Settings};
use crate::config_util;
use crate::config_util::FlattenKeyword;

type HighlightFn = Box<dyn Fn(&str) -> String + Send>;
type HighlightFnVec = Vec<HighlightFn>;

pub struct Highlighters {
    pub before: HighlightFnVec,
    pub after: HighlightFnVec,
}

impl Highlighters {
    pub fn new(config: Config) -> Highlighters {
        let mut before_fns: HighlightFnVec = Vec::new();
        let mut after_fns: HighlightFnVec = Vec::new();

        if let Some(numbers_style) = &config.groups.numbers {
            before_fns.push(numbers::highlight(numbers_style));
        }

        // Keywords
        let flattened_keywords = Self::flatten(&config);
        for keyword in flattened_keywords {
            before_fns.push(keyword::highlight(keyword.keyword, &keyword.style));
        }

        if let Some(quotes_style) = &config.groups.quotes {
            after_fns.push(quotes::highlight(
                quotes_style,
                config.settings.quotes_token,
            ));
        }

        Highlighters {
            before: before_fns,
            after: after_fns,
        }
    }

    fn flatten(config: &Config) -> Vec<FlattenKeyword> {
        let keywords_or_empty = config.groups.keywords.clone().unwrap_or_default();

        config_util::flatten_keywords(keywords_or_empty)
    }
}
