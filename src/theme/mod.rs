use serde::Deserialize;
use tailspin::config::*;

pub mod reader;

/// `theme.toml` as written by the user. Every table deserializes directly
/// into the core config struct it styles.
#[derive(Deserialize, Debug, Default)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    pub keywords: Vec<KeywordConfig>,
    pub regexes: Vec<RegexConfig>,
    pub numbers: NumberConfig,
    pub uuids: UuidConfig,
    pub quotes: QuoteConfig,
    pub ipv4: IpV4Config,
    pub ipv6: IpV6Config,
    pub dates: DateTimeConfig,
    pub durations: DurationConfig,
    pub paths: UnixPathConfig,
    pub urls: UrlConfig,
    pub emails: EmailConfig,
    pub pointers: PointerConfig,
    pub processes: UnixProcessConfig,
    pub key_value_pairs: KeyValueConfig,
    pub json: JsonConfig,
    pub jvm_stack_traces: JvmStackTraceConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tailspin::style::{Color, Style};

    fn parse(input: &str) -> Theme {
        toml::from_str::<Theme>(input).unwrap()
    }

    #[test]
    fn empty_input_yields_defaults() {
        let theme = parse("");

        assert!(theme.keywords.is_empty());
        assert!(theme.regexes.is_empty());
        assert_eq!(theme.uuids.letter, UuidConfig::default().letter);
        assert_eq!(theme.quotes.quote_token, b'"');
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
        assert!(toml::from_str::<Theme>("[uuids]\nnumber = { fg = \"red\", itallic = true }").is_err());
        // The pre-7.0 key for the numbers style.
        assert!(toml::from_str::<Theme>("[numbers]\nnumber = { fg = \"red\" }").is_err());
    }

    #[test]
    fn ipv4_and_ipv6_tables_parse_independently() {
        let theme = parse(
            r#"[ipv4]
separator = { fg = "yellow" }

[ipv6]
letter = { fg = "green" }"#,
        );

        assert_eq!(theme.ipv4.separator, Style::new().fg(Color::Yellow));
        assert_eq!(theme.ipv4.number, IpV4Config::default().number);
        assert_eq!(theme.ipv6.letter, Style::new().fg(Color::Green));
        assert_eq!(theme.ipv6.separator, IpV6Config::default().separator);
    }

    #[test]
    fn the_removed_ip_addresses_table_is_rejected() {
        let error = toml::from_str::<Theme>("[ip_addresses]\nseparator = { fg = \"red\" }").unwrap_err();

        assert!(error.to_string().contains("ip_addresses"));
    }

    #[test]
    fn numbers_table_uses_the_style_key() {
        let theme = parse(
            r#"[numbers]
style = { fg = "green" }"#,
        );

        assert_eq!(theme.numbers.style, Style::new().fg(Color::Green));
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

        assert_eq!(theme.quotes.quote_token, b'\'');
        assert_eq!(theme.quotes.style, QuoteConfig::default().style);
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
