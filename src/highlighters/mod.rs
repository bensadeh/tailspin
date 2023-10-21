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
use crate::theme::defaults::get_default_keywords;
use crate::theme::{Keyword, Theme};
use crate::types::Highlight;

pub struct Highlighters {
    pub before: Vec<Box<dyn Highlight + Send>>,
    pub main: Vec<Box<dyn Highlight + Send>>,
    pub after: Vec<Box<dyn Highlight + Send>>,
}

impl Highlighters {
    pub fn new(theme: &Theme) -> Highlighters {
        Highlighters {
            before: Self::set_before_fns(theme),
            main: Self::set_main_fns(theme),
            after: Self::set_after_fns(theme),
        }
    }

    fn set_before_fns(theme: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut before_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if !theme.date.disabled {
            before_fns.push(Box::new(DateHighlighter::new(
                &theme.date.date,
                &theme.date.time,
                &theme.date.zone,
            )));
        }

        if !theme.url.disabled {
            before_fns.push(Box::new(UrlHighlighter::new(&theme.url)));
        }

        if !theme.path.disabled {
            before_fns.push(Box::new(PathHighlighter::new(
                &theme.path.segment,
                &theme.path.separator,
            )));
        }

        if !theme.ip.disabled {
            before_fns.push(Box::new(IpHighlighter::new(&theme.ip.segment, &theme.ip.separator)));
        }

        if !theme.key_value.disabled {
            before_fns.push(Box::new(KeyValueHighlighter::new(
                &theme.key_value.key,
                &theme.key_value.separator,
            )));
        }

        if !theme.uuid.disabled {
            before_fns.push(Box::new(UuidHighlighter::new(
                &theme.uuid.segment,
                &theme.uuid.separator,
            )));
        }

        if !theme.process.disabled {
            before_fns.push(Box::new(ProcessHighlighter::new(
                &theme.process.name,
                &theme.process.separator,
                &theme.process.id,
            )));
        }

        before_fns
    }

    fn set_main_fns(theme: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut main_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();
        let keywords = Self::get_keywords(&theme.keywords, true);

        if !theme.number.disabled {
            main_fns.push(Box::new(NumberHighlighter::new(&theme.number.style)));
        }

        for keyword in keywords {
            main_fns.push(Box::new(KeywordHighlighter::new(
                &keyword.words,
                &keyword.style,
                keyword.border,
            )));
        }

        main_fns
    }

    fn set_after_fns(theme: &Theme) -> Vec<Box<dyn Highlight + Send>> {
        let mut after_fns: Vec<Box<dyn Highlight + Send>> = Vec::new();

        if !theme.quotes.disabled {
            after_fns.push(Box::new(QuoteHighlighter::new(&theme.quotes.style, theme.quotes.token)));
        }

        after_fns
    }

    fn get_keywords(custom_keywords: &Option<Vec<Keyword>>, disable_default_keywords: bool) -> Vec<Keyword> {
        if disable_default_keywords {
            let default_keywords = get_default_keywords();
            match custom_keywords {
                Some(ck) => [default_keywords, ck.clone()].concat(),
                None => default_keywords,
            }
        } else {
            custom_keywords.clone().unwrap_or_default()
        }
    }
}
