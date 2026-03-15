use std::borrow::Cow;
use tailspin::config::*;
use tailspin::style::{Color, Style};
use tailspin::*;

mod utils;

#[test]
fn test_binary_with_various_inputs() {
    let binary_path = utils::build_binary();

    let test_cases = [
        ("Hello null", "Hello \u{1b}[3;31mnull\u{1b}[0m"),
        ("Hello world", "Hello world"),
        ("", ""),
    ];

    for (input, expected_output) in test_cases {
        let output = utils::run_binary_with_input(binary_path.clone(), input);
        assert_eq!(output.trim(), expected_output, "Failed on input: {}", input);
    }
}

#[test]
fn default_constructor_should_not_panic() {
    let result = std::panic::catch_unwind(Highlighter::default);

    assert!(result.is_ok(), "Default constructor should never fail");
}

#[test]
fn no_highlights_should_return_borrowed() {
    let highlighter = Highlighter::default();

    // Each input bypasses progressively more fast-path checks (byte-level early
    // returns) while still not matching any highlighter regex, so the pipeline
    // must return Cow::Borrowed for every one of them.
    let inputs: &[&str] = &[
        // No trigger characters — every fast-path returns early.
        "Nothing will be highlighted in this string",
        // Colon present → DateTime and IpV6 run their regex.
        "status: pending",
        // Dot present → IpV4 runs its regex.
        "hello.world",
        // Dash and slash → DateDash and UnixPath run their regex.
        "left-right mid/end",
        // Bracket present → UnixProcess runs its regex.
        "see [note] here",
        // Equals present → KeyValue runs its regex.
        "not ==> equal",
        // Contains 'x' → Pointer runs its regex.
        "extra context",
        // All trigger characters present — every highlighter reaches its regex.
        //   :  → DateTime, IpV6         .  → IpV4
        //   -  → DateDash (×4 for UUID) /  → UnixPath
        //   [  → UnixProcess            =  → KeyValue
        //   x  → Pointer
        "mix: [note] x.y a-b-c-d-e ==> w/q",
    ];

    for input in inputs {
        let output = highlighter.apply(input);
        assert!(
            matches!(output, Cow::Borrowed(s) if std::ptr::eq(s, *input)),
            "Expected Cow::Borrowed for input: {input:?}, got Cow::Owned",
        );
    }
}

#[test]
fn it_works() {
    let mut builder = Highlighter::builder();

    builder
        .with_number_highlighter(NumberConfig {
            style: Style {
                fg: Some(Color::Cyan),
                ..Style::default()
            },
        })
        .with_quote_highlighter(QuotesConfig {
            quotes_token: b'"',
            style: Style {
                fg: Some(Color::Yellow),
                ..Style::default()
            },
        })
        .with_uuid_highlighter(UuidConfig::default());

    let highlighter = match builder.build() {
        Ok(h) => h,
        Err(_) => panic!("Failed to build highlighter"),
    };

    let actual = highlighter.apply("Hello 123 world! ");
    let expected = "Hello \u{1b}[36m123\u{1b}[0m world! ".to_string();

    assert_eq!(actual, expected);
}
