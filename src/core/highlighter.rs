use crate::core::config::*;
use crate::core::span_pipeline::Pipeline;
use crate::core::span_pipeline::finders::date_dash::DateDashFinder;
use crate::core::span_pipeline::finders::date_time::DateTimeFinder;
use crate::core::span_pipeline::finders::email::EmailFinder;
use crate::core::span_pipeline::finders::ip_v4::IpV4Finder;
use crate::core::span_pipeline::finders::ip_v6::IpV6Finder;
use crate::core::span_pipeline::finders::json::JsonFinder;
use crate::core::span_pipeline::finders::jvm_stack::JvmStackFinder;
use crate::core::span_pipeline::finders::key_value::KeyValueFinder;
use crate::core::span_pipeline::finders::keyword::KeywordFinder;
use crate::core::span_pipeline::finders::number::NumberFinder;
use crate::core::span_pipeline::finders::pointer::PointerFinder;
use crate::core::span_pipeline::finders::quote::QuoteFinder;
use crate::core::span_pipeline::finders::regex::RegexFinder;
use crate::core::span_pipeline::finders::unix_path::UnixPathFinder;
use crate::core::span_pipeline::finders::unix_process::UnixProcessFinder;
use crate::core::span_pipeline::finders::url::UrlFinder;
use crate::core::span_pipeline::finders::uuid::UuidFinder;
use crate::core::span_pipeline::span::Finder;
use crate::style::Style;
use std::borrow::Cow;
use std::collections::HashMap;
use thiserror::Error;

/// A pattern-based log highlighter.
///
/// `Highlighter` applies configured pattern-based highlighters to text inputs,
/// returning highlighted output with ANSI colors.
#[derive(Debug)]
pub struct Highlighter {
    inner: Pipeline,
}

/// An error produced while building a [`Highlighter`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A custom regex pattern failed to compile.
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// A keyword set could not be compiled into a keyword searcher.
    #[error("Pattern error: {0}")]
    Pattern(#[from] aho_corasick::BuildError),
}

impl Highlighter {
    /// Creates a new [`HighlighterBuilder`] for configuring a [`Highlighter`].
    pub const fn builder() -> HighlighterBuilder {
        HighlighterBuilder {
            finders: Vec::new(),
            first_error: None,
        }
    }

    /// Applies the configured highlights to the given input string.
    #[must_use]
    pub fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.inner.apply(input)
    }
}

impl Default for Highlighter {
    /// Creates a default `Highlighter` with common patterns.
    ///
    /// This operation is expensive and should be done once and reused.
    fn default() -> Self {
        Highlighter::builder()
            .with_json_highlighter(JsonConfig::default())
            .with_date_time_highlighters(DateTimeConfig::default())
            .with_url_highlighter(UrlConfig::default())
            .with_email_highlighter(EmailConfig::default())
            .with_ip_v4_highlighter(IpV4Config::default())
            .with_uuid_highlighter(UuidConfig::default())
            .with_pointer_highlighter(PointerConfig::default())
            .with_unix_path_highlighter(UnixPathConfig::default())
            .with_unix_process_highlighter(UnixProcessConfig::default())
            .with_key_value_highlighter(KeyValueConfig::default())
            .with_number_highlighter(NumberConfig::default())
            .with_quote_highlighter(QuoteConfig::default())
            .build()
            .expect("Default constructor should never fail")
    }
}

/// Builder for configuring a [`Highlighter`].
#[derive(Debug)]
#[must_use]
pub struct HighlighterBuilder {
    finders: Vec<Box<dyn Finder>>,
    first_error: Option<Error>,
}

impl HighlighterBuilder {
    /// Adds a highlighter for numbers.
    pub fn with_number_highlighter(mut self, config: NumberConfig) -> Self {
        self.add_finder(NumberFinder::new(config));
        self
    }

    /// Adds a highlighter for UUIDs.
    pub fn with_uuid_highlighter(mut self, config: UuidConfig) -> Self {
        self.add_finder(UuidFinder::new(config));
        self
    }

    /// Adds a highlighter for Unix file paths.
    pub fn with_unix_path_highlighter(mut self, config: UnixPathConfig) -> Self {
        self.add_finder(UnixPathFinder::new(config));
        self
    }

    /// Adds a highlighter for Unix processes.
    pub fn with_unix_process_highlighter(mut self, config: UnixProcessConfig) -> Self {
        self.add_finder(UnixProcessFinder::new(config));
        self
    }

    /// Adds a highlighter for key-value pairs.
    pub fn with_key_value_highlighter(mut self, config: KeyValueConfig) -> Self {
        self.add_finder(KeyValueFinder::new(config));
        self
    }

    /// Adds highlighters for dates and times.
    pub fn with_date_time_highlighters(mut self, config: DateTimeConfig) -> Self {
        self.add_finder(DateTimeFinder::new(config));
        self.add_finder(DateDashFinder::new(config));
        self
    }

    /// Adds a highlighter for IPv6 addresses.
    pub fn with_ip_v6_highlighter(mut self, config: IpV6Config) -> Self {
        self.add_finder(IpV6Finder::new(config));
        self
    }

    /// Adds a highlighter for JVM stack traces (Java, Kotlin, Scala, etc.).
    pub fn with_jvm_stack_trace_highlighter(mut self, config: JvmStackTraceConfig) -> Self {
        self.add_finder(JvmStackFinder::new(config));
        self
    }

    /// Adds a highlighter for IPv4 addresses.
    pub fn with_ip_v4_highlighter(mut self, config: IpV4Config) -> Self {
        self.add_finder(IpV4Finder::new(config));
        self
    }

    /// Adds a highlighter for URLs.
    pub fn with_url_highlighter(mut self, config: UrlConfig) -> Self {
        self.add_finder(UrlFinder::new(config));
        self
    }

    /// Adds a highlighter for email addresses.
    pub fn with_email_highlighter(mut self, config: EmailConfig) -> Self {
        self.add_finder(EmailFinder::new(config));
        self
    }

    /// Adds a highlighter for memory pointers.
    pub fn with_pointer_highlighter(mut self, config: PointerConfig) -> Self {
        self.add_finder(PointerFinder::new(config));
        self
    }

    /// Adds a highlighter using a custom regex pattern.
    pub fn with_regex_highlighter(mut self, config: RegexConfig) -> Self {
        self.try_add_finder(RegexFinder::new(&config.regex, config.style).map_err(Error::Regex));
        self
    }

    /// Adds a highlighter for quoted text.
    pub fn with_quote_highlighter(mut self, config: QuoteConfig) -> Self {
        self.add_finder(QuoteFinder::new(config));
        self
    }

    /// Adds a highlighter for JSON structures.
    pub fn with_json_highlighter(mut self, config: JsonConfig) -> Self {
        self.add_finder(JsonFinder::new(config));
        self
    }

    /// Adds keyword highlighters.
    pub fn with_keyword_highlighter(mut self, keyword_configs: Vec<KeywordConfig>) -> Self {
        for keyword_config in group_keywords_by_style(keyword_configs) {
            let word_refs: Vec<&str> = keyword_config.words.iter().map(String::as_str).collect();
            self.try_add_finder(KeywordFinder::new(&word_refs, keyword_config.style).map_err(Error::Pattern));
        }

        self
    }

    /// Finalizes the builder and returns a configured [`Highlighter`].
    pub fn build(self) -> Result<Highlighter, Error> {
        if let Some(err) = self.first_error {
            Err(err)
        } else {
            Ok(Highlighter {
                inner: Pipeline::new(self.finders),
            })
        }
    }

    fn add_finder<F: Finder + 'static>(&mut self, finder: F) {
        self.try_add_finder(Ok(finder));
    }

    fn try_add_finder<F: Finder + 'static>(&mut self, finder: Result<F, Error>) {
        if self.first_error.is_some() {
            return;
        }
        match finder {
            Ok(f) => self.finders.push(Box::new(f)),
            Err(e) => self.first_error = Some(e),
        }
    }
}

fn group_keywords_by_style(configs: Vec<KeywordConfig>) -> Vec<KeywordConfig> {
    let mut grouped_configs: HashMap<Style, Vec<String>> = HashMap::new();

    for config in configs {
        grouped_configs.entry(config.style).or_default().extend(config.words);
    }

    let mut result: Vec<KeywordConfig> = grouped_configs
        .into_iter()
        .map(|(style, mut words)| {
            words.sort();
            KeywordConfig { words, style }
        })
        .collect();

    result.sort_by_key(|a| a.style);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    fn number_then_quote_highlighter() -> Highlighter {
        Highlighter::builder()
            .with_number_highlighter(NumberConfig {
                style: Style::new().fg(Color::Cyan),
            })
            .with_quote_highlighter(QuoteConfig {
                quote_token: b'"',
                style: Style::new().fg(Color::Yellow),
            })
            .build()
            .unwrap()
    }

    #[test]
    fn test_quote_highlights_around_existing_number() {
        let highlighter = number_then_quote_highlighter();

        let input = r#"count is "value 42 here" end"#;
        // In the span pipeline, number (priority 0) wins over quote (priority 1)
        // so number is cyan, quote fills the gaps with yellow
        let expected = r#"count is [yellow]"value [reset][cyan]42[reset][yellow] here"[reset] end"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_quote_with_no_highlights_inside() {
        let highlighter = number_then_quote_highlighter();

        let input = r#"msg "hello world" end"#;
        let expected = r#"msg [yellow]"hello world"[reset] end"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_number_outside_quotes_unaffected() {
        let highlighter = number_then_quote_highlighter();

        let input = r#"code 200 "error" end"#;
        let expected = r#"code [cyan]200[reset] [yellow]"error"[reset] end"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_multiple_numbers_inside_quotes() {
        let highlighter = number_then_quote_highlighter();

        let input = r#""port 8080 and 443""#;
        let expected = r#"[yellow]"port [reset][cyan]8080[reset][yellow] and [reset][cyan]443[reset][yellow]"[reset]"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_multiple_quoted_segments() {
        let highlighter = number_then_quote_highlighter();

        let input = r#""count 1" and "count 2""#;
        let expected = r#"[yellow]"count [reset][cyan]1[reset][yellow]"[reset] and [yellow]"count [reset][cyan]2[reset][yellow]"[reset]"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_no_quotes_only_numbers() {
        let highlighter = number_then_quote_highlighter();

        let input = "status 200 ok";
        let expected = "status [cyan]200[reset] ok";

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn invalid_regex_fails_build_and_first_error_wins() {
        let result = Highlighter::builder()
            .with_regex_highlighter(RegexConfig {
                regex: "(unclosed".to_string(),
                style: Style::default(),
            })
            .with_keyword_highlighter(vec![kw(&["ok"], Style::default())])
            .build();

        assert!(matches!(result, Err(Error::Regex(_))));
    }

    fn kw(words: &[&str], style: Style) -> KeywordConfig {
        KeywordConfig {
            words: words.iter().map(ToString::to_string).collect(),
            style,
        }
    }

    #[test]
    fn groups_same_style_and_sorts_words() {
        let red_bold = Style::new().fg(Color::Red).bold();
        let green_underline = Style::new().fg(Color::Green).underline();

        let configs = vec![
            kw(&["hello", "world"], red_bold),
            kw(&["foo", "bar"], red_bold),
            kw(&["baz"], green_underline),
        ];

        let expected = vec![
            kw(&["bar", "foo", "hello", "world"], red_bold),
            kw(&["baz"], green_underline),
        ];

        assert_eq!(group_keywords_by_style(configs), expected);
    }

    #[test]
    fn grouping_empty_input_returns_empty() {
        assert!(group_keywords_by_style(vec![]).is_empty());
    }

    #[test]
    fn does_not_group_slightly_different_styles() {
        let configs = vec![
            kw(&["error"], Style::new().fg(Color::Red).bold()),
            kw(&["null"], Style::new().fg(Color::Red).italic()),
        ];

        let expected = vec![
            kw(&["null"], Style::new().fg(Color::Red).italic()),
            kw(&["error"], Style::new().fg(Color::Red).bold()),
        ];

        assert_eq!(group_keywords_by_style(configs), expected);
    }

    #[test]
    fn single_group_passes_through() {
        let configs = vec![kw(&["unique"], Style::new().fg(Color::Blue).italic())];

        assert_eq!(group_keywords_by_style(configs.clone()), configs);
    }
}
