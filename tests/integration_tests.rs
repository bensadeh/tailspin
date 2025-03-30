use tailspin::config::*;
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
            quotes_token: '"',
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
