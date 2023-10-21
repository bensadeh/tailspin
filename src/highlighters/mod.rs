mod date;
mod ip;
mod key_value;
mod keyword;
mod number;
mod path;
mod process;
mod quotes;
mod url;
mod uuid;

use crate::highlighters::date::DateHighlighter;
use crate::highlighters::ip::IpHighlighter;
use crate::highlighters::key_value::KeyValueHighlighter;
use crate::highlighters::keyword::KeywordHighlighter;
use crate::highlighters::number::NumberHighlighter;
use crate::highlighters::path::PathHighlighter;
use crate::highlighters::process::ProcessHighlighter;
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
    pub fn new(config: Theme) -> Highlighters {
        Highlighters {
            before: Self::set_before_fns(&config),
            main: Self::set_main_fns(&config),
            after: Self::set_after_fns(&config),
        }
    }

    fn set_before_fns(config: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut before_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(dates) = &config.groups.date {
            if !dates.disabled {
                before_fns.push(Box::new(DateHighlighter::new(&dates.date, &dates.time, &dates.zone)));
            }
        }

        if let Some(url) = &config.groups.url {
            if !url.disabled {
                before_fns.push(Box::new(UrlHighlighter::new(url)));
            }
        }

        if let Some(path) = &config.groups.path {
            if !path.disabled {
                before_fns.push(Box::new(PathHighlighter::new(&path.segment, &path.separator)));
            }
        }

        if let Some(ip) = &config.groups.ip {
            if !ip.disabled {
                before_fns.push(Box::new(IpHighlighter::new(&ip.segment, &ip.separator)));
            }
        }

        if let Some(kv) = &config.groups.key_value {
            if !kv.disabled {
                before_fns.push(Box::new(KeyValueHighlighter::new(&kv.key, &kv.separator)));
            }
        }

        if !config.groups.uuid.disabled {
            before_fns.push(Box::new(UuidHighlighter::new(
                &config.groups.uuid.segment,
                &config.groups.uuid.separator,
            )));
        }

        if let Some(p) = &config.groups.process {
            if !p.disabled {
                before_fns.push(Box::new(ProcessHighlighter::new(&p.name, &p.separator, &p.id)));
            }
        }

        before_fns
    }

    fn set_main_fns(config: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut main_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(numbers) = &config.groups.number {
            if !numbers.disabled {
                main_fns.push(Box::new(NumberHighlighter::new(&numbers.style)));
            }
        }

        if let Some(keywords) = &config.groups.keywords {
            for keyword in keywords {
                main_fns.push(Box::new(KeywordHighlighter::new(
                    &keyword.words,
                    &keyword.style,
                    keyword.border,
                )));
            }
        }

        main_fns
    }

    fn set_after_fns(config: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut after_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(quotes_group) = &config.groups.quotes {
            if !quotes_group.disabled {
                after_fns.push(Box::new(QuoteHighlighter::new(&quotes_group.style, quotes_group.token)));
            }
        }

        after_fns
    }
}
