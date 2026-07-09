//! Property tests for the pipeline's text-preservation invariant: stripping
//! the ANSI codes from highlighted output recovers the input, except that a
//! background-styled keyword surviving merge intact gains one padding space
//! on each side (a "badge").
//!
//! The first property uses a badge-free configuration and demands the input
//! back byte-for-byte; the second enables badges and checks, space-
//! insensitively, that nothing but spaces changed.

use proptest::prelude::*;
use std::sync::LazyLock;
use tailspin::Highlighter;
use tailspin::config::*;
use tailspin::style::{Color, Style};

fn full_highlighter(keyword_style: Style) -> Highlighter {
    Highlighter::builder()
        .with_json_highlighter(JsonConfig::default())
        .with_regex_highlighter(RegexConfig {
            regex: r"\btrace-\d+\b".to_string(),
            style: Style::new().fg(Color::Magenta),
        })
        .with_date_time_highlighters(DateTimeConfig::default())
        .with_ip_v4_highlighter(IpV4Config::default())
        .with_ip_v6_highlighter(IpV6Config::default())
        .with_jvm_stack_trace_highlighter(JvmStackTraceConfig::default())
        .with_url_highlighter(UrlConfig::default())
        .with_email_highlighter(EmailConfig::default())
        .with_unix_path_highlighter(UnixPathConfig::default())
        .with_duration_highlighter(DurationConfig::default())
        .with_key_value_highlighter(KeyValueConfig::default())
        .with_uuid_highlighter(UuidConfig::default())
        .with_pointer_highlighter(PointerConfig::default())
        .with_unix_process_highlighter(UnixProcessConfig::default())
        .with_number_highlighter(NumberConfig::default())
        .with_keyword_highlighter(vec![KeywordConfig {
            words: vec!["ERROR".to_string(), "GET".to_string(), "null".to_string()],
            style: keyword_style,
        }])
        .with_quote_highlighter(QuoteConfig::default())
        .build()
        .unwrap()
}

static PLAIN: LazyLock<Highlighter> = LazyLock::new(|| full_highlighter(Style::new().fg(Color::Red)));
static BADGED: LazyLock<Highlighter> = LazyLock::new(|| full_highlighter(Style::new().fg(Color::White).on(Color::Red)));

/// Removes every SGR sequence (`ESC [ ... m`) the renderer emits.
fn strip_sgr(styled: &str) -> String {
    let mut out = String::with_capacity(styled.len());
    let mut rest = styled;
    while let Some(esc) = rest.find('\x1b') {
        out.push_str(&rest[..esc]);
        let tail = &rest[esc..];
        let end = tail.find('m').expect("unterminated SGR sequence");
        rest = &tail[end + 1..];
    }
    out.push_str(rest);
    out
}

const FRAGMENTS: &[&str] = &[
    "connection",
    "established",
    "ERROR",
    "GET",
    "null",
    "true",
    "trace-42",
    "3.14",
    "150ms",
    "2.5s",
    "550e8400-e29b-41d4-a716-446655440000",
    "0xdeadbeef",
    "0xd7b3b2f446e2c21b",
    "192.168.0.1/24",
    "2001:db8::ff00:42:8329",
    "https://example.com/a_(b)?key=val&x=2",
    "user@sub.example.co.uk",
    "/var/log/nginx/error.log",
    "~/projects/tailspin",
    "postfix/smtp[1894]",
    "key=value",
    "\"quoted text\"",
    "'single'",
    r#"{"a": 1, "items": [true, null], "s": "va\"lue"}"#,
    "2024-09-14T07:57:30.659Z",
    "2022-09-09 11:48:34,534",
    "09/30/2022",
    "java.io.IOException: pipe closed\n        at com.foo.Bar.<init>(Bar.java:42)",
    "        ... 42 more",
];

fn fragment() -> impl Strategy<Value = String> {
    prop_oneof![
        4 => prop::sample::select(FRAGMENTS).prop_map(str::to_string),
        1 => any::<u32>().prop_map(|n| n.to_string()),
        1 => (any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>()).prop_map(|(a, b, c, d)| format!("{a}.{b}.{c}.{d}")),
        1 => any::<String>().prop_map(|s| s.replace('\x1b', "")),
    ]
}

fn line() -> impl Strategy<Value = String> {
    (
        prop::collection::vec(fragment(), 0..12),
        prop::sample::select(&[" ", "\n"]),
    )
        .prop_map(|(fragments, separator)| fragments.join(separator))
}

proptest! {
    #[test]
    fn stripping_ansi_recovers_the_input(input in line()) {
        let output = PLAIN.apply(&input);
        prop_assert_eq!(strip_sgr(&output), input);
    }

    #[test]
    fn badge_padding_only_ever_adds_spaces(input in line()) {
        let output = BADGED.apply(&input);
        prop_assert_eq!(strip_sgr(&output).replace(' ', ""), input.replace(' ', ""));
    }
}
