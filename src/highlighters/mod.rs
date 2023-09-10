mod date;
mod ip;
mod keyword;
mod number;
mod path;
mod quotes;
mod url;
mod uuid;

use crate::highlighters::date::DateHighlighter;
use crate::highlighters::ip::IpHighlighter;
use crate::highlighters::keyword::KeywordHighlighter;
use crate::highlighters::number::NumberHighlighter;
use crate::highlighters::path::PathHighlighter;
use crate::highlighters::quotes::QuoteHighlighter;
use crate::highlighters::url::UrlHighlighter;
use crate::highlighters::uuid::UuidHighlighter;
use crate::theme::Theme;
use crate::types::Highlight;

pub struct Highlighters {
    pub before: Vec<Box<dyn Highlight + Send>>,
    pub main: Vec<Box<dyn Highlight + Send>>,
    pub after: Vec<Box<dyn Highlight + Send>>,
}

impl Highlighters {
    fn set_before_fns(config: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut before_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(dates) = &config.groups.date {
            before_fns.push(Box::new(DateHighlighter::new(
                &dates.date,
                &dates.time,
                &dates.zone,
            )));
        }

        if let Some(url) = &config.groups.url {
            before_fns.push(Box::new(UrlHighlighter::new(url)));
        }

        if let Some(path) = &config.groups.path {
            before_fns.push(Box::new(PathHighlighter::new(
                &path.segment,
                &path.separator,
            )));
        }

        if let Some(ip) = &config.groups.ip {
            before_fns.push(Box::new(IpHighlighter::new(&ip.segment, &ip.separator)));
        }

        if let Some(uuid) = &config.groups.uuid {
            before_fns.push(Box::new(UuidHighlighter::new(
                &uuid.segment,
                &uuid.separator,
            )));
        }

        before_fns
    }

    fn set_main_fns(config: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut main_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(numbers) = &config.groups.number {
            main_fns.push(Box::new(NumberHighlighter::new(&numbers.style)));
        }

        if let Some(keywords) = &config.groups.keywords {
            for keyword in keywords {
                main_fns.push(Box::new(KeywordHighlighter::new(
                    &keyword.words,
                    &keyword.style,
                )));
            }
        }

        main_fns
    }

    fn set_after_fns(config: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut after_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(quotes_group) = &config.groups.quotes {
            after_fns.push(Box::new(QuoteHighlighter::new(
                &quotes_group.style,
                quotes_group.token,
            )));
        }

        after_fns
    }

    pub fn new(config: Theme) -> Highlighters {
        Highlighters {
            before: Self::set_before_fns(&config),
            main: Self::set_main_fns(&config),
            after: Self::set_after_fns(&config),
        }
    }
}
