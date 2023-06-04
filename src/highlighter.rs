use regex::Regex;

pub struct Highlighter {}

impl Highlighter {
    pub fn new() -> Highlighter {
        Highlighter {}
    }

    pub fn apply(&self, input: &str) -> String {
        let highlighted_string = self.highlight_numbers_in_blue(input);
        self.highlight_quotes(highlighted_string.as_str())
    }

    fn highlight_numbers_in_blue(&self, input: &str) -> String {
        let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

        let highlighted = number_regex.replace_all(input, |caps: &regex::Captures<'_>| {
            format!("\x1B[34m{}\x1B[0m", &caps[0])
        });

        highlighted.into_owned()
    }

    fn highlight_quotes(&self, input: &str) -> String {
        let quote_count: usize = input.chars().filter(|&ch| ch == '"').count();
        if quote_count % 2 != 0 {
            return input.to_string();
        }

        let mut output = String::new();
        let mut inside_quote = false;
        let mut potential_color_code = String::new();

        let yellow = "\x1b[33m";
        let reset = "\x1b[0m";

        for ch in input.chars() {
            if ch == '"' {
                inside_quote = !inside_quote;
                if inside_quote {
                    output.push_str(yellow);
                    output.push(ch);
                } else {
                    output.push(ch);
                    output.push_str(reset);
                }
                continue;
            }

            if inside_quote {
                potential_color_code.push(ch);

                if potential_color_code == reset {
                    output.push_str(&potential_color_code);
                    output.push_str(yellow);
                    potential_color_code.clear();
                } else if !reset.starts_with(&potential_color_code) {
                    output.push_str(&potential_color_code);
                    potential_color_code.clear();
                }
            } else {
                output.push(ch);
            }
        }

        output
    }
}
