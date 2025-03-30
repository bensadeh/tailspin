use crate::highlighter::config::*;
use crate::highlighter::highlighters::date_dash::DateDashHighlighter;
use crate::highlighter::highlighters::date_time::TimeHighlighter;
use crate::highlighter::highlighters::ip_v4::IpV4Highlighter;
use crate::highlighter::highlighters::ip_v6::IpV6Highlighter;
use crate::highlighter::highlighters::json::JsonHighlighter;
use crate::highlighter::highlighters::key_value::KeyValueHighlighter;
use crate::highlighter::highlighters::keyword::KeywordHighlighter;
use crate::highlighter::highlighters::number::NumberHighlighter;
use crate::highlighter::highlighters::pointer::PointerHighlighter;
use crate::highlighter::highlighters::quote::QuoteHighlighter;
use crate::highlighter::highlighters::regex::RegexpHighlighter;
use crate::highlighter::highlighters::unix_path::UnixPathHighlighter;
use crate::highlighter::highlighters::unix_process::UnixProcessHighlighter;
use crate::highlighter::highlighters::url::UrlHighlighter;
use crate::highlighter::highlighters::uuid::UuidHighlighter;
use crate::highlighter::normalizer::normalize_keyword_configs;
use crate::highlighter::split_and_apply::apply_only_to_unhighlighted;
use crate::Error;
use std::sync::Arc;

pub trait Highlight: Sync + Send {
    fn apply(&self, input: &str) -> String;
}

pub struct Highlighter {
    highlighters: Vec<Arc<dyn Highlight>>,
}

impl Highlighter {
    const fn new() -> Self {
        Highlighter {
            highlighters: Vec::new(),
        }
    }

    pub fn builder() -> HighlightBuilder {
        HighlightBuilder {
            highlighters: Vec::new(),
            regex_errors: Vec::new(),
        }
    }

    fn with_highlighters(mut self, highlighters: Vec<Arc<dyn Highlight>>) -> Self {
        self.highlighters = highlighters;

        self
    }

    pub fn apply(&self, input: &str) -> String {
        self.highlighters.iter().fold(input.to_owned(), |acc, highlighter| {
            apply_only_to_unhighlighted(&acc, highlighter)
        })
    }
}

impl Default for Highlighter {
    /// Compiles a default highlighter with reasonable defaults.
    ///
    /// Since we are compiling regexes under the hood, this is an expensive operation and should be done once and then
    /// be reused.
    fn default() -> Self {
        let mut builder = Highlighter::builder();

        builder
            .with_json_highlighter(JsonConfig::default())
            .with_date_time_highlighters(DateTimeConfig::default())
            .with_url_highlighter(UrlConfig::default())
            .with_ip_v4_highlighter(IpV4Config::default())
            .with_ip_v6_highlighter(IpV6Config::default())
            .with_uuid_highlighter(UuidConfig::default())
            .with_pointer_highlighter(PointerConfig::default())
            .with_unix_path_highlighter(UnixPathConfig::default())
            .with_unix_process_highlighter(UnixProcessConfig::default())
            .with_key_value_highlighter(KeyValueConfig::default())
            .with_number_highlighter(NumberConfig::default())
            .with_quote_highlighter(QuotesConfig::default());

        builder.build().expect("Default constructor should never fail")
    }
}

pub struct HighlightBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
    regex_errors: Vec<regex::Error>,
}

impl HighlightBuilder {
    pub fn with_number_highlighter(&mut self, config: NumberConfig) -> &mut Self {
        self.try_add_highlighter(NumberHighlighter::new(config));
        self
    }

    pub fn with_uuid_highlighter(&mut self, config: UuidConfig) -> &mut Self {
        self.try_add_highlighter(UuidHighlighter::new(config));
        self
    }

    pub fn with_unix_path_highlighter(&mut self, config: UnixPathConfig) -> &mut Self {
        self.try_add_highlighter(UnixPathHighlighter::new(config));
        self
    }

    pub fn with_unix_process_highlighter(&mut self, config: UnixProcessConfig) -> &mut Self {
        self.try_add_highlighter(UnixProcessHighlighter::new(config));
        self
    }

    pub fn with_key_value_highlighter(&mut self, config: KeyValueConfig) -> &mut Self {
        self.try_add_highlighter(KeyValueHighlighter::new(config))
    }

    pub fn with_date_time_highlighters(&mut self, config: DateTimeConfig) -> &mut Self {
        self.try_add_highlighter(TimeHighlighter::new(config))
            .try_add_highlighter(DateDashHighlighter::new(config))
    }

    pub fn with_ip_v6_highlighter(&mut self, config: IpV6Config) -> &mut Self {
        self.try_add_highlighter(IpV6Highlighter::new(config));
        self
    }

    pub fn with_ip_v4_highlighter(&mut self, config: IpV4Config) -> &mut Self {
        self.try_add_highlighter(IpV4Highlighter::new(config));
        self
    }

    pub fn with_url_highlighter(&mut self, config: UrlConfig) -> &mut Self {
        self.try_add_highlighter(UrlHighlighter::new(config));
        self
    }

    pub fn with_pointer_highlighter(&mut self, config: PointerConfig) -> &mut Self {
        self.try_add_highlighter(PointerHighlighter::new(config));
        self
    }

    pub fn with_regex_highlighter(&mut self, config: RegexConfig) -> &mut Self {
        self.try_add_highlighter(RegexpHighlighter::new(config));
        self
    }

    pub fn with_quote_highlighter(&mut self, config: QuotesConfig) -> &mut Self {
        self.try_add_highlighter(Ok(QuoteHighlighter::new(config)));
        self
    }

    pub fn with_json_highlighter(&mut self, config: JsonConfig) -> &mut Self {
        self.try_add_highlighter(Ok(JsonHighlighter::new(config)));
        self
    }

    pub fn with_keyword_highlighter(&mut self, keyword_configs: Vec<KeywordConfig>) -> &mut Self {
        let normalized_keyword_configs = normalize_keyword_configs(keyword_configs);

        for keyword_config in normalized_keyword_configs {
            let highlighter = KeywordHighlighter::new(keyword_config);

            match highlighter {
                Ok(h) => self.highlighters.push(Arc::new(h)),
                Err(e) => self.regex_errors.push(e),
            }
        }

        self
    }

    fn try_add_highlighter<T: Highlight + 'static>(&mut self, highlighter: Result<T, regex::Error>) -> &mut Self {
        match highlighter {
            Ok(h) => self.highlighters.push(Arc::new(h)),
            Err(e) => self.regex_errors.push(e),
        }

        self
    }

    pub fn build(self) -> Result<Highlighter, Error> {
        match self.regex_errors.is_empty() {
            true => Ok(Highlighter::new().with_highlighters(self.highlighters)),
            false => Err(Error::RegexErrors(self.regex_errors)),
        }
    }
}
