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
use crate::theme::Groups;
use crate::types::Highlight;

pub struct Highlighters {
    pub before: Vec<Box<dyn Highlight + Send>>,
    pub main: Vec<Box<dyn Highlight + Send>>,
    pub after: Vec<Box<dyn Highlight + Send>>,
}

impl Highlighters {
    pub fn new(groups: &Groups) -> Highlighters {
        Highlighters {
            before: Self::set_before_fns(groups),
            main: Self::set_main_fns(groups),
            after: Self::set_after_fns(groups),
        }
    }

    fn set_before_fns(groups: &Groups) -> Vec<Box<dyn Highlight + Send>> {
        let mut before_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if !groups.date.disabled {
            before_fns.push(Box::new(DateHighlighter::new(
                &groups.date.date,
                &groups.date.time,
                &groups.date.zone,
            )));
        }

        if let Some(url) = &groups.url {
            if !url.disabled {
                before_fns.push(Box::new(UrlHighlighter::new(url)));
            }
        }

        if !groups.path.disabled {
            before_fns.push(Box::new(PathHighlighter::new(
                &groups.path.segment,
                &groups.path.separator,
            )));
        }

        if !groups.ip.disabled {
            before_fns.push(Box::new(IpHighlighter::new(&groups.ip.segment, &groups.ip.separator)));
        }

        if !groups.key_value.disabled {
            before_fns.push(Box::new(KeyValueHighlighter::new(
                &groups.key_value.key,
                &groups.key_value.separator,
            )));
        }

        if !groups.uuid.disabled {
            before_fns.push(Box::new(UuidHighlighter::new(
                &groups.uuid.segment,
                &groups.uuid.separator,
            )));
        }

        if let Some(p) = &groups.process {
            if !p.disabled {
                before_fns.push(Box::new(ProcessHighlighter::new(&p.name, &p.separator, &p.id)));
            }
        }

        before_fns
    }

    fn set_main_fns(groups: &Groups) -> Vec<Box<dyn Highlight + Send>> {
        let mut main_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(numbers) = &groups.number {
            if !numbers.disabled {
                main_fns.push(Box::new(NumberHighlighter::new(&numbers.style)));
            }
        }

        if let Some(keywords) = &groups.keywords {
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

    fn set_after_fns(groups: &Groups) -> Vec<Box<dyn Highlight + Send>> {
        let mut after_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if let Some(quotes_group) = &groups.quotes {
            if !quotes_group.disabled {
                after_fns.push(Box::new(QuoteHighlighter::new(&quotes_group.style, quotes_group.token)));
            }
        }

        after_fns
    }
}
