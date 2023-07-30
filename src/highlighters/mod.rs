mod date;
mod ip;
mod keyword;
mod number;
mod path;
mod quotes;
mod url;
mod uuid;

use crate::line_info::LineInfo;
use crate::theme::Keyword;
use crate::theme::Style;
use crate::theme::Theme;

type HighlightFn = Box<dyn Fn(&str, &LineInfo) -> String + Send>;
type HighlightFnVec = Vec<HighlightFn>;

pub struct Highlighters {
    pub before: HighlightFnVec,
    pub main: HighlightFnVec,
    pub after: HighlightFnVec,
}

struct FlattenKeyword {
    pub keyword: String,
    pub style: Style,
}

impl Highlighters {
    pub fn new(config: Theme) -> Highlighters {
        let mut before_fns: HighlightFnVec = Vec::new();
        let mut main_fns: HighlightFnVec = Vec::new();
        let mut after_fns: HighlightFnVec = Vec::new();

        // Dates
        if let Some(dates) = &config.groups.date {
            before_fns.push(date::highlight(&dates.style));
        }

        // URLs
        if let Some(url) = &config.groups.url {
            before_fns.push(url::highlight(url));
        }

        // Paths
        if let Some(path) = &config.groups.path {
            before_fns.push(path::highlight(&path.segment, &path.separator));
        }

        // IPs
        if let Some(ip) = &config.groups.ip {
            before_fns.push(ip::highlight(&ip.segment, &ip.separator));
        }

        // UUIDs
        if let Some(uuid) = &config.groups.uuid {
            before_fns.push(uuid::highlight(&uuid.segment, &uuid.separator));
        }

        // Numbers
        if let Some(numbers) = &config.groups.number {
            main_fns.push(number::highlight(&numbers.style));
        }

        // Keywords
        let flattened_keywords = Self::flatten(&config);
        let keyword_strings: Vec<String> = flattened_keywords
            .iter()
            .map(|kw| kw.keyword.clone())
            .collect();

        keyword::init_keywords(keyword_strings);

        for keyword in flattened_keywords {
            main_fns.push(keyword::highlight(keyword.keyword, &keyword.style));
        }

        // Quotes
        if let Some(quotes_group) = &config.groups.quotes {
            after_fns.push(quotes::highlight(&quotes_group.style, quotes_group.token));
        }

        Highlighters {
            before: before_fns,
            main: main_fns,
            after: after_fns,
        }
    }

    fn flatten(config: &Theme) -> Vec<FlattenKeyword> {
        let keywords_or_empty = config.groups.keywords.clone().unwrap_or_default();

        Self::flatten_keywords(keywords_or_empty)
    }

    fn flatten_keywords(keywords: Vec<Keyword>) -> Vec<FlattenKeyword> {
        let mut flatten_keywords = Vec::new();

        for keyword in keywords {
            for string in keyword.words {
                flatten_keywords.push(FlattenKeyword {
                    keyword: string,
                    style: keyword.style.clone(),
                });
            }
        }

        flatten_keywords
    }
}
