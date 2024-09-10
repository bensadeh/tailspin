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
        assert_eq!(output.trim(), expected_output, "Failed on input: {input}");
    }
}
