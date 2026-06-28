use serde::Deserialize;
use tailspin::config::*;
use tailspin::style::Style;

pub mod reader;

/// `theme.toml` as written by the user. Most tables deserialize directly into
/// the core config structs; the `*Toml` wrappers below exist only where the
/// TOML shape differs from the config struct it produces. The builder converts
/// those wrappers at point of use (see `cli::highlighter`).
#[derive(Deserialize, Debug, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub keywords: Vec<KeywordConfig>,
    pub regexes: Vec<RegexConfig>,
    pub numbers: NumberToml,
    pub uuids: UuidConfig,
    pub quotes: QuotesToml,
    pub ip_addresses: IpToml,
    pub dates: DateTimeConfig,
    pub paths: UnixPathConfig,
    pub urls: UrlConfig,
    pub emails: EmailConfig,
    pub pointers: PointerConfig,
    pub processes: UnixProcessConfig,
    pub key_value_pairs: KeyValueConfig,
    pub json: JsonConfig,
    pub jvm_stack_traces: JvmStackTraceConfig,
}

/// `[numbers]` styles its single field under the key `number`, while
/// `NumberConfig` calls it `style`.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct NumberToml {
    pub number: Option<Style>,
}

impl From<NumberToml> for NumberConfig {
    fn from(toml: NumberToml) -> Self {
        NumberConfig {
            style: toml.number.unwrap_or(NumberConfig::default().style),
        }
    }
}

/// `[quotes]` takes the quote character as a `char`, while `QuoteConfig`
/// stores an ASCII byte. Non-ASCII characters are rejected at parse time.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct QuotesToml {
    #[serde(default, deserialize_with = "ascii_char")]
    pub quote_token: Option<u8>,
    pub style: Option<Style>,
}

fn ascii_char<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let ch = char::deserialize(deserializer)?;

    if ch.is_ascii() {
        Ok(Some(ch as u8))
    } else {
        Err(serde::de::Error::custom(format!(
            "quote_token must be an ASCII character, got `{ch}`"
        )))
    }
}

impl From<QuotesToml> for QuoteConfig {
    fn from(toml: QuotesToml) -> Self {
        let default = QuoteConfig::default();

        QuoteConfig {
            quote_token: toml.quote_token.unwrap_or(default.quote_token),
            style: toml.style.unwrap_or(default.style),
        }
    }
}

/// The single `[ip_addresses]` table styles both IPv4 and IPv6 addresses;
/// `letter` only applies to IPv6.
#[derive(Deserialize, Debug, Default, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct IpToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub separator: Option<Style>,
}

impl From<IpToml> for IpV4Config {
    fn from(toml: IpToml) -> Self {
        let default = IpV4Config::default();

        IpV4Config {
            number: toml.number.unwrap_or(default.number),
            separator: toml.separator.unwrap_or(default.separator),
        }
    }
}

impl From<IpToml> for IpV6Config {
    fn from(toml: IpToml) -> Self {
        let default = IpV6Config::default();

        IpV6Config {
            number: toml.number.unwrap_or(default.number),
            letter: toml.letter.unwrap_or(default.letter),
            separator: toml.separator.unwrap_or(default.separator),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailspin::style::Color;

    fn parse(input: &str) -> Theme {
        toml::from_str::<Theme>(input).unwrap()
    }

    #[test]
    fn empty_input_yields_defaults() {
        let theme = parse("");

        assert!(theme.keywords.is_empty());
        assert!(theme.regexes.is_empty());
        assert_eq!(theme.uuids.letter, UuidConfig::default().letter);
        assert_eq!(QuoteConfig::from(theme.quotes).quote_token, b'"');
    }

    #[test]
    fn partial_table_keeps_defaults_for_missing_fields() {
        let theme = parse(
            r#"[uuids]
number = { fg = "red" }"#,
        );

        assert_eq!(theme.uuids.number, Style::new().fg(Color::Red));
        assert_eq!(theme.uuids.letter, UuidConfig::default().letter);
        assert_eq!(theme.uuids.separator, UuidConfig::default().separator);
    }

    #[test]
    fn unknown_keys_are_rejected() {
        assert!(toml::from_str::<Theme>("bogus = 1").is_err());
        assert!(toml::from_str::<Theme>("[uuids]\nbogus = { fg = \"red\" }").is_err());
    }

    #[test]
    fn ip_addresses_table_styles_both_v4_and_v6() {
        let theme = parse(
            r#"[ip_addresses]
separator = { fg = "yellow" }"#,
        );

        assert_eq!(
            IpV4Config::from(theme.ip_addresses).separator,
            Style::new().fg(Color::Yellow)
        );
        assert_eq!(
            IpV6Config::from(theme.ip_addresses).separator,
            Style::new().fg(Color::Yellow)
        );
        assert_eq!(
            IpV6Config::from(theme.ip_addresses).letter,
            IpV6Config::default().letter
        );
    }

    #[test]
    fn numbers_table_uses_the_number_key() {
        let theme = parse(
            r#"[numbers]
number = { fg = "green" }"#,
        );

        assert_eq!(NumberConfig::from(theme.numbers).style, Style::new().fg(Color::Green));
    }

    #[test]
    fn keywords_and_regexes_parse_into_config_lists() {
        let theme = parse(
            r#"[[keywords]]
words = ["foo"]
style = { bold = true }

[[regexes]]
regex = "x+"
style = { fg = "blue" }"#,
        );

        assert_eq!(
            theme.keywords,
            vec![KeywordConfig {
                words: vec!["foo".to_string()],
                style: Style::new().bold(),
            }]
        );
        assert_eq!(theme.regexes[0].regex, "x+");
    }

    #[test]
    fn ascii_quote_token_is_accepted() {
        let theme = parse(
            r#"[quotes]
quote_token = "'""#,
        );

        assert_eq!(QuoteConfig::from(theme.quotes).quote_token, b'\'');
    }

    #[test]
    fn non_ascii_quote_token_is_rejected() {
        let error = toml::from_str::<Theme>(
            r#"[quotes]
quote_token = "«""#,
        )
        .unwrap_err();

        assert!(error.to_string().contains("ASCII"));
    }
}
