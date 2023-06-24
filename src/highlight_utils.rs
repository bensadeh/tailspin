use crate::color;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref ESCAPE_CODE_REGEX: Regex =
        Regex::new(r"\x1b\[[0-9;]*m").expect("Invalid regex pattern");
}

pub(crate) fn highlight_with_awareness(color: &str, input: &str, regex: &Regex) -> String {
    let chunks = split_into_chunks(input);

    let mut output = String::new();
    for chunk in chunks {
        match chunk {
            Chunk::Normal(text) => {
                let highlighted = regex.replace_all(text, |caps: &Captures<'_>| {
                    format!("{}{}{}", color, &caps[0], color::RESET)
                });
                output.push_str(&highlighted);
            }
            Chunk::Highlighted(text) => {
                output.push_str(text);
            }
        }
    }

    output
}

enum Chunk<'a> {
    Normal(&'a str),
    Highlighted(&'a str),
}

fn split_into_chunks(input: &str) -> Vec<Chunk> {
    let reset_code = "\x1b[0m";

    let mut rest = input;
    let mut inside_escape = false;
    let mut chunks = Vec::new();

    while !rest.is_empty() {
        if !inside_escape {
            if let Some(mat) = ESCAPE_CODE_REGEX.find(rest) {
                let (before_escape, from_escape) = rest.split_at(mat.start());
                chunks.push(Chunk::Normal(before_escape));
                rest = from_escape;
                inside_escape = true;
            } else {
                chunks.push(Chunk::Normal(rest));
                rest = "";
            }
        } else if let Some(reset_position) = rest.find(reset_code) {
            let (escape_code, remaining) = rest.split_at(reset_position + reset_code.len());
            chunks.push(Chunk::Highlighted(escape_code));
            rest = remaining;
            inside_escape = false;
        } else {
            chunks.push(Chunk::Highlighted(rest));
            rest = "";
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_highlight_with_awareness() {
        let regex = Regex::new(r"\b\d+\b").unwrap();
        let input = "Here is a number 12345, and here is another 54321.";
        let color = "\x1b[31m"; // ANSI color code for red
        let result = highlight_with_awareness(color, input, &regex);

        assert_eq!(
            result,
            "Here is a number \x1b[31m12345\x1b[0m, and here is another \x1b[31m54321\x1b[0m."
        );
    }

    #[test]
    fn test_highlight_with_awareness_with_highlighted_chunks() {
        let regex = Regex::new(r"\b\d+\b").unwrap();
        let input = "Here is a date \x1b[31m2023-06-24\x1b[0m, and here is a number 12345.";
        let color = "\x1b[31m"; // ANSI color code for red
        let result = highlight_with_awareness(color, input, &regex);

        assert_eq!(
            result,
            "Here is a date \x1b[31m2023-06-24\x1b[0m, and here is a number \x1b[31m12345\x1b[0m."
        );
    }

    #[test]
    fn test_split_into_chunks() {
        let input = "Here is a date \x1b[31m2023-06-24\x1b[0m, and here is a number 12345.";
        let chunks = split_into_chunks(input);

        assert_eq!(chunks.len(), 3);
        match &chunks[0] {
            Chunk::Normal(text) => assert_eq!(*text, "Here is a date "),
            _ => panic!("Unexpected chunk type."),
        }
        match &chunks[1] {
            Chunk::Highlighted(text) => assert_eq!(*text, "\x1b[31m2023-06-24\x1b[0m"),
            _ => panic!("Unexpected chunk type."),
        }
        match &chunks[2] {
            Chunk::Normal(text) => assert_eq!(*text, ", and here is a number 12345."),
            _ => panic!("Unexpected chunk type."),
        }
    }
}
