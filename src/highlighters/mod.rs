mod date;
mod keyword;
mod number;
mod quotes;
mod url;
mod uuid;

use crate::config_parser::Config;
use crate::config_util;
use crate::config_util::FlattenKeyword;

type HighlightFn = Box<dyn Fn(&str) -> String + Send>;
type HighlightFnVec = Vec<HighlightFn>;

pub struct Highlighters {
    pub before: HighlightFnVec,
    pub main: HighlightFnVec,
    pub after: HighlightFnVec,
}

impl Highlighters {
    pub fn new(config: Config) -> Highlighters {
        let mut before_fns: HighlightFnVec = Vec::new();
        let mut main_fns: HighlightFnVec = Vec::new();
        let mut after_fns: HighlightFnVec = Vec::new();

        // Dates
        if let Some(dates_style) = &config.groups.dates {
            before_fns.push(date::highlight(dates_style));
        }

        // URLs
        if let Some(url_group) = &config.groups.urls {
            before_fns.push(url::highlight(url_group));
        }

        // UUIDs
        if let Some(uuids) = &config.groups.uuids {
            before_fns.push(uuid::highlight(&uuids.segment, &uuids.separator));
        }

        // Numbers
        if let Some(numbers_style) = &config.groups.numbers {
            main_fns.push(number::highlight(numbers_style));
        }

        // Keywords
        let flattened_keywords = Self::flatten(&config);
        for keyword in flattened_keywords {
            main_fns.push(keyword::highlight(keyword.keyword, &keyword.style));
        }

        // Quotes
        if let Some(quotes_style) = &config.groups.quotes {
            after_fns.push(quotes::highlight(
                quotes_style,
                config.settings.quotes_token,
            ));
        }

        Highlighters {
            before: before_fns,
            main: main_fns,
            after: after_fns,
        }
    }

    fn flatten(config: &Config) -> Vec<FlattenKeyword> {
        let keywords_or_empty = config.groups.keywords.clone().unwrap_or_default();

        config_util::flatten_keywords(keywords_or_empty)
    }
}
