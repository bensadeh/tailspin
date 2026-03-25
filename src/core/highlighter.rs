use crate::core::config::*;
use crate::core::highlighters::date_dash::DateDashHighlighter;
use crate::core::highlighters::date_time::DateTimeHighlighter;
use crate::core::highlighters::email::EmailHighlighter;
use crate::core::highlighters::ip_v4::IpV4Highlighter;
use crate::core::highlighters::ip_v6::IpV6Highlighter;
use crate::core::highlighters::json::JsonHighlighter;
use crate::core::highlighters::key_value::KeyValueHighlighter;
use crate::core::highlighters::keyword::KeywordHighlighter;
use crate::core::highlighters::number::NumberHighlighter;
use crate::core::highlighters::pointer::PointerHighlighter;
use crate::core::highlighters::quote::QuoteHighlighter;
use crate::core::highlighters::regex::RegexHighlighter;
use crate::core::highlighters::unix_path::UnixPathHighlighter;
use crate::core::highlighters::unix_process::UnixProcessHighlighter;
use crate::core::highlighters::url::UrlHighlighter;
use crate::core::highlighters::uuid::UuidHighlighter;
use crate::core::utils::normalizer::normalize_keyword_configs;
use crate::core::utils::split_and_apply::apply_only_to_unhighlighted;
use std::borrow::Cow;
use thiserror::Error;

/// A regex-based log highlighter.
///
/// `Highlighter` applies configured regex-based highlighters to text inputs,
/// returning highlighted output with ANSI colors.
pub struct Highlighter {
    highlighters: Vec<Box<dyn Highlight>>,
}

pub trait Highlight: Sync + Send {
    fn apply<'a>(&self, input: &'a str) -> Cow<'a, str>;

    fn apply_to_line<'a>(&self, input: &'a str) -> Cow<'a, str> {
        apply_only_to_unhighlighted(input, self)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Regex error: {0}")]
    RegexError(String),

    #[error("Pattern error: {0}")]
    PatternError(String),
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
            first_error: None,
        }
    }

    fn with_highlighters(mut self, highlighters: Vec<Box<dyn Highlight>>) -> Self {
        self.highlighters = highlighters;
        self
    }

    /// Applies the configured highlights to the given input string.
    #[must_use]
    pub fn apply<'a>(&self, input: &'a str) -> Cow<'a, str> {
        self.highlighters.iter().fold(Cow::Borrowed(input), |acc, highlighter| {
            let result = highlighter.apply_to_line(&acc);
            match result {
                Cow::Borrowed(_) => acc,
                Cow::Owned(modified) => Cow::Owned(modified),
            }
        })
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
#[must_use]
pub struct HighlighterBuilder {
    highlighters: Vec<Box<dyn Highlight>>,
    first_error: Option<Error>,
}

impl HighlighterBuilder {
    /// Adds a highlighter for numbers.
    pub fn with_number_highlighter(mut self, config: NumberConfig) -> Self {
        self.add_highlighter(NumberHighlighter::new(config));
        self
    }

    /// Adds a highlighter for UUIDs.
    pub fn with_uuid_highlighter(mut self, config: UuidConfig) -> Self {
        self.add_highlighter(UuidHighlighter::new(config));
        self
    }

    /// Adds a highlighter for Unix file paths.
    pub fn with_unix_path_highlighter(mut self, config: UnixPathConfig) -> Self {
        self.add_highlighter(UnixPathHighlighter::new(config));
        self
    }

    /// Adds a highlighter for Unix processes.
    pub fn with_unix_process_highlighter(mut self, config: UnixProcessConfig) -> Self {
        self.add_highlighter(UnixProcessHighlighter::new(config));
        self
    }

    /// Adds a highlighter for key-value pairs.
    pub fn with_key_value_highlighter(mut self, config: KeyValueConfig) -> Self {
        self.add_highlighter(KeyValueHighlighter::new(config));
        self
    }

    /// Adds highlighters for dates and times.
    pub fn with_date_time_highlighters(mut self, config: DateTimeConfig) -> Self {
        self.add_highlighter(DateTimeHighlighter::new(config));
        self.add_highlighter(DateDashHighlighter::new(config));
        self
    }

    /// Adds a highlighter for IPv6 addresses.
    pub fn with_ip_v6_highlighter(mut self, config: IpV6Config) -> Self {
        self.add_highlighter(IpV6Highlighter::new(config));
        self
    }

    /// Adds a highlighter for IPv4 addresses.
    pub fn with_ip_v4_highlighter(mut self, config: IpV4Config) -> Self {
        self.add_highlighter(IpV4Highlighter::new(config));
        self
    }

    /// Adds a highlighter for URLs.
    pub fn with_url_highlighter(mut self, config: UrlConfig) -> Self {
        self.add_highlighter(UrlHighlighter::new(config));
        self
    }

    /// Adds a highlighter for email addresses.
    pub fn with_email_highlighter(mut self, config: EmailConfig) -> Self {
        self.add_highlighter(EmailHighlighter::new(config));
        self
    }

    /// Adds a highlighter for memory pointers.
    pub fn with_pointer_highlighter(mut self, config: PointerConfig) -> Self {
        self.add_highlighter(PointerHighlighter::new(config));
        self
    }

    /// Adds a highlighter using a custom regex pattern.
    pub fn with_regex_highlighter(mut self, config: RegexConfig) -> Self {
        self.try_add_highlighter(RegexHighlighter::new(config));
        self
    }

    /// Adds a highlighter for quoted text.
    pub fn with_quote_highlighter(mut self, config: QuoteConfig) -> Self {
        self.add_highlighter(QuoteHighlighter::new(config));
        self
    }

    /// Adds a highlighter for JSON structures.
    pub fn with_json_highlighter(mut self, config: JsonConfig) -> Self {
        self.add_highlighter(JsonHighlighter::new(config));
        self
    }

    /// Adds keyword highlighters.
    pub fn with_keyword_highlighter(mut self, keyword_configs: Vec<KeywordConfig>) -> Self {
        let normalized_keyword_configs = normalize_keyword_configs(keyword_configs);

        for keyword_config in normalized_keyword_configs {
            if self.first_error.is_some() {
                continue;
            }

            match KeywordHighlighter::new(keyword_config) {
                Ok(h) => self.highlighters.push(Box::new(h)),
                Err(e) => self.first_error = Some(Error::PatternError(e.to_string())),
            }
        }

        self
    }

    /// Finalizes the builder and returns a configured [`Highlighter`].
    pub fn build(self) -> Result<Highlighter, Error> {
        if let Some(err) = self.first_error {
            Err(err)
        } else {
            Ok(Highlighter::new().with_highlighters(self.highlighters))
        }
    }

    fn add_highlighter<H: Highlight + 'static>(&mut self, highlighter: H) {
        if self.first_error.is_none() {
            self.highlighters.push(Box::new(highlighter));
        }
    }

    fn try_add_highlighter<H: Highlight + 'static>(&mut self, highlighter: Result<H, regex::Error>) {
        if self.first_error.is_some() {
            return;
        }

        match highlighter {
            Ok(h) => self.highlighters.push(Box::new(h)),
            Err(e) => self.first_error = Some(Error::RegexError(e.to_string())),
        }
    }
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
        let expected = r#"count is [yellow]"value [cyan]42[reset][yellow] here"[reset] end"#;

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
        let expected = r#"[yellow]"port [cyan]8080[reset][yellow] and [cyan]443[reset][yellow]"[reset]"#;

        let actual = highlighter.apply(input);

        assert_eq!(actual.to_string().convert_escape_codes(), expected);
    }

    #[test]
    fn test_multiple_quoted_segments() {
        let highlighter = number_then_quote_highlighter();

        let input = r#""count 1" and "count 2""#;
        let expected =
            r#"[yellow]"count [cyan]1[reset][yellow]"[reset] and [yellow]"count [cyan]2[reset][yellow]"[reset]"#;

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
}
