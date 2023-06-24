mod keyword;
mod numbers;
mod quotes;

use crate::color::{Fg, RESET};
use crate::config_parser::{Config, Settings};
use crate::config_util::FlattenKeyword;

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

        before_fns.push(numbers::highlight(color_for_numbers.to_string()));

        // iterate over keywords and push them to before_fns
        // for keyword in keywords {
        //     before_fns.push(keyword.keyword, keyword.highlight);
        // }

        // before_fns.push(keyword::highlight(Fg::Red.to_string(), "null".to_string()));

        // let quotes = config.groups.quotes;

        if let Some(quotes_style) = config.groups.quotes {
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
}
