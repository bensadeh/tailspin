mod date;
mod ip;
mod key_value;
mod keyword;
mod number;
mod path;
mod process;
mod quotes;
mod time;
mod url;
mod uuid;

use crate::cli::Cli;
use crate::highlighters::date::DateHighlighter;
use crate::highlighters::ip::IpHighlighter;
use crate::highlighters::key_value::KeyValueHighlighter;
use crate::highlighters::keyword::KeywordHighlighter;
use crate::highlighters::number::NumberHighlighter;
use crate::highlighters::path::PathHighlighter;
use crate::highlighters::process::ProcessHighlighter;
use crate::highlighters::quotes::QuoteHighlighter;
use crate::highlighters::time::TimeHighlighter;
use crate::highlighters::url::UrlHighlighter;
use crate::highlighters::uuid::UuidHighlighter;
use crate::keyword::consolidator::consolidate_keywords;
use crate::keyword::extractor::extract_all_keywords;
use crate::theme::defaults::{get_boolean_keywords, get_rest_keywords, get_severity_keywords};
use crate::theme::{Keyword, Theme};
use crate::types::Highlight;
use std::sync::Arc;

pub struct Highlighters {
    pub before: Vec<Arc<dyn Highlight + Send + Sync>>,
    pub main: Vec<Arc<dyn Highlight + Send + Sync>>,
    pub after: Vec<Arc<dyn Highlight + Send + Sync>>,
}

impl Highlighters {
    pub fn new(theme: &Theme, cli: &Cli) -> Highlighters {
        Highlighters {
            before: Self::set_before_fns(theme),
            main: Self::set_main_fns(theme, cli),
            after: Self::set_after_fns(theme),
        }
    }

    fn set_before_fns(theme: &Theme) -> Vec<Arc<dyn Highlight + Send + Sync>> {
        let mut before_fns: Vec<Arc<dyn Highlight + Send + Sync>> = Vec::new();

        if !theme.date.disabled {
            before_fns.push(Arc::new(DateHighlighter::new(
                &theme.date.style,
                theme.date.shorten.clone(),
            )));
        }

        if !theme.time.disabled {
            before_fns.push(Arc::new(TimeHighlighter::new(
                &theme.time.time,
                &theme.time.zone,
                theme.time.shorten.clone(),
            )));
        }

        if !theme.url.disabled {
            before_fns.push(Arc::new(UrlHighlighter::new(&theme.url)));
        }

        if !theme.path.disabled {
            before_fns.push(Arc::new(PathHighlighter::new(
                &theme.path.segment,
                &theme.path.separator,
            )));
        }

        if !theme.ip.disabled {
            before_fns.push(Arc::new(IpHighlighter::new(&theme.ip.segment, &theme.ip.separator)));
        }

        if !theme.key_value.disabled {
            before_fns.push(Arc::new(KeyValueHighlighter::new(
                &theme.key_value.key,
                &theme.key_value.separator,
            )));
        }

        if !theme.uuid.disabled {
            before_fns.push(Arc::new(UuidHighlighter::new(
                &theme.uuid.segment,
                &theme.uuid.separator,
            )));
        }

        if !theme.process.disabled {
            before_fns.push(Arc::new(ProcessHighlighter::new(
                &theme.process.name,
                &theme.process.separator,
                &theme.process.id,
            )));
        }

        before_fns
    }

    fn set_main_fns(theme: &Theme, cli: &Cli) -> Vec<Arc<dyn Highlight + Send + Sync>> {
        let mut main_fns: Vec<Arc<dyn Highlight + Send + Sync>> = Vec::new();
        let keywords = Self::get_keywords(theme, cli);

        if !theme.number.disabled {
            main_fns.push(Arc::new(NumberHighlighter::new(&theme.number.style)));
        }

        for keyword in keywords {
            main_fns.push(Arc::new(KeywordHighlighter::new(
                &keyword.words,
                &keyword.style,
                keyword.border,
            )));
        }

        main_fns
    }

    fn set_after_fns(theme: &Theme) -> Vec<Arc<dyn Highlight + Send + Sync>> {
        let mut after_fns: Vec<Arc<dyn Highlight + Send + Sync>> = Vec::new();

        if !theme.quotes.disabled {
            after_fns.push(Arc::new(QuoteHighlighter::new(&theme.quotes.style, theme.quotes.token)));
        }

        after_fns
    }

    fn get_keywords(theme: &Theme, cli: &Cli) -> Vec<Keyword> {
        let custom_and_builtins = Self::get_custom_and_builtin_keywords(theme, cli);
        let on_the_fly_keywords = extract_all_keywords(
            cli.words_red.clone(),
            cli.words_green.clone(),
            cli.words_yellow.clone(),
            cli.words_blue.clone(),
            cli.words_magenta.clone(),
            cli.words_cyan.clone(),
        );

        let all_keywords = [custom_and_builtins, on_the_fly_keywords].concat();

        consolidate_keywords(all_keywords)
    }

    fn get_custom_and_builtin_keywords(theme: &Theme, cli: &Cli) -> Vec<Keyword> {
        let mut all_keywords = theme.keywords.clone().unwrap_or_default();

        if !cli.disable_keyword_builtins {
            if !cli.disable_booleans {
                all_keywords.extend(get_boolean_keywords());
            }

            if !cli.disable_severity {
                all_keywords.extend(get_severity_keywords());
            }

            if !cli.disable_rest {
                all_keywords.extend(get_rest_keywords());
            }
        }

        all_keywords
    }
}
