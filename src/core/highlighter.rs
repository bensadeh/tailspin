use crate::Error;
use crate::core::config::*;
use crate::core::highlighters::StaticHighlight;
use crate::core::highlighters::date_dash::DateDashHighlighter;
use crate::core::highlighters::date_time::TimeHighlighter;
use crate::core::highlighters::ip_v4::IpV4Highlighter;
use crate::core::highlighters::ip_v6::IpV6Highlighter;
use crate::core::highlighters::json::JsonHighlighter;
use crate::core::highlighters::key_value::KeyValueHighlighter;
use crate::core::highlighters::keyword::KeywordHighlighter;
use crate::core::highlighters::number::NumberHighlighter;
use crate::core::highlighters::pointer::PointerHighlighter;
use crate::core::highlighters::quote::QuoteHighlighter;
use crate::core::highlighters::regex::RegexpHighlighter;
use crate::core::highlighters::unix_path::UnixPathHighlighter;
use crate::core::highlighters::unix_process::UnixProcessHighlighter;
use crate::core::highlighters::url::UrlHighlighter;
use crate::core::highlighters::uuid::UuidHighlighter;
use crate::core::utils::normalizer::normalize_keyword_configs;
use crate::core::utils::split_and_apply::apply_only_to_unhighlighted;
use std::borrow::Cow;

pub trait Highlight: Sync + Send {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str>;
}

/// A regex-based log highlighter.
///
/// `Highlighter` applies configured regex-based highlighters to text inputs,
/// returning highlighted output with ANSI colors.
pub struct Highlighter {
    highlighters: Vec<StaticHighlight>,
}

impl Highlighter {
    const fn new() -> Self {
        Highlighter {
            highlighters: Vec::new(),
        }
    }

    /// Creates a new [`HighlighterBuilder`] for configuring a [`Highlighter`].
    pub const fn builder() -> HighlighterBuilder {
        HighlighterBuilder {
            highlighters: Vec::new(),
            regex_errors: Vec::new(),
        }
    }

    fn with_highlighters(mut self, highlighters: Vec<StaticHighlight>) -> Self {
        self.highlighters = highlighters;
        self
    }

    /// Applies the configured highlights to the given input string.
    pub fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.highlighters.iter().fold(
            Cow::Borrowed(input),
            |acc, highlighter| match apply_only_to_unhighlighted(&acc, highlighter) {
                Cow::Borrowed(_) => acc,
                Cow::Owned(modified) => Cow::Owned(modified),
            },
        )
    }
}

impl Default for Highlighter {
    /// Creates a default `Highlighter` with common patterns.
    ///
    /// This operation is expensive and should be done once and reused.
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

/// Builder for configuring a [`Highlighter`].
pub struct HighlighterBuilder {
    highlighters: Vec<StaticHighlight>,
    regex_errors: Vec<regex::Error>,
}

impl HighlighterBuilder {
    /// Adds a highlighter for numbers.
    pub fn with_number_highlighter(&mut self, config: NumberConfig) -> &mut Self {
        self.try_add_highlighter(NumberHighlighter::new(config).map(StaticHighlight::Number));
        self
    }

    /// Adds a highlighter for UUIDs.
    pub fn with_uuid_highlighter(&mut self, config: UuidConfig) -> &mut Self {
        self.try_add_highlighter(UuidHighlighter::new(config).map(StaticHighlight::Uuid));
        self
    }

    /// Adds a highlighter for Unix file paths.
    pub fn with_unix_path_highlighter(&mut self, config: UnixPathConfig) -> &mut Self {
        self.try_add_highlighter(UnixPathHighlighter::new(config).map(StaticHighlight::UnixPath));
        self
    }

    /// Adds a highlighter for Unix processes.
    pub fn with_unix_process_highlighter(&mut self, config: UnixProcessConfig) -> &mut Self {
        self.try_add_highlighter(UnixProcessHighlighter::new(config).map(StaticHighlight::UnixProcess));
        self
    }

    /// Adds a highlighter for key-value pairs.
    pub fn with_key_value_highlighter(&mut self, config: KeyValueConfig) -> &mut Self {
        self.try_add_highlighter(KeyValueHighlighter::new(config).map(StaticHighlight::KeyValue));
        self
    }

    /// Adds highlighters for dates and times.
    pub fn with_date_time_highlighters(&mut self, config: DateTimeConfig) -> &mut Self {
        self.try_add_highlighter(TimeHighlighter::new(config).map(StaticHighlight::Time))
            .try_add_highlighter(DateDashHighlighter::new(config).map(StaticHighlight::DateDash))
    }

    /// Adds a highlighter for IPv6 addresses.
    pub fn with_ip_v6_highlighter(&mut self, config: IpV6Config) -> &mut Self {
        self.try_add_highlighter(IpV6Highlighter::new(config).map(StaticHighlight::IpV6));
        self
    }

    /// Adds a highlighter for IPv4 addresses.
    pub fn with_ip_v4_highlighter(&mut self, config: IpV4Config) -> &mut Self {
        self.try_add_highlighter(IpV4Highlighter::new(config).map(StaticHighlight::IpV4));
        self
    }

    /// Adds a highlighter for URLs.
    pub fn with_url_highlighter(&mut self, config: UrlConfig) -> &mut Self {
        self.try_add_highlighter(UrlHighlighter::new(config).map(StaticHighlight::Url));
        self
    }

    /// Adds a highlighter for memory pointers.
    pub fn with_pointer_highlighter(&mut self, config: PointerConfig) -> &mut Self {
        self.try_add_highlighter(PointerHighlighter::new(config).map(StaticHighlight::Pointer));
        self
    }

    /// Adds a highlighter using a custom regex pattern.
    pub fn with_regex_highlighter(&mut self, config: RegexConfig) -> &mut Self {
        self.try_add_highlighter(RegexpHighlighter::new(config).map(StaticHighlight::Regexp));
        self
    }

    /// Adds a highlighter for quoted text.
    pub fn with_quote_highlighter(&mut self, config: QuotesConfig) -> &mut Self {
        self.try_add_highlighter(Ok(StaticHighlight::Quote(QuoteHighlighter::new(config))));
        self
    }

    /// Adds a highlighter for JSON structures.
    pub fn with_json_highlighter(&mut self, config: JsonConfig) -> &mut Self {
        self.try_add_highlighter(Ok(StaticHighlight::Json(JsonHighlighter::new(config))));
        self
    }

    /// Adds keyword highlighters.
    pub fn with_keyword_highlighter(&mut self, keyword_configs: Vec<KeywordConfig>) -> &mut Self {
        let normalized_keyword_configs = normalize_keyword_configs(keyword_configs);

        for keyword_config in normalized_keyword_configs {
            let highlighter = KeywordHighlighter::new(keyword_config);

            match highlighter {
                Ok(h) => self.highlighters.push(StaticHighlight::Keyword(h)),
                Err(e) => self.regex_errors.push(e),
            }
        }

        self
    }

    /// Finalizes the builder and returns a configured [`Highlighter`].
    pub fn build(self) -> Result<Highlighter, Error> {
        match self.regex_errors.is_empty() {
            true => Ok(Highlighter::new().with_highlighters(self.highlighters)),
            false => Err(Error::RegexErrors(self.regex_errors)),
        }
    }

    fn try_add_highlighter(&mut self, highlighter: Result<StaticHighlight, regex::Error>) -> &mut Self {
        match highlighter {
            Ok(h) => self.highlighters.push(h),
            Err(e) => self.regex_errors.push(e),
        }
        self
    }
}
