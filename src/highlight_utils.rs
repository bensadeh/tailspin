use crate::color;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref ESCAPE_CODE_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").expect("Invalid regex pattern");
}

const MAX_ALLOCATION_SIZE: usize = 1024 * 1024; // 1 MiB

pub(crate) fn replace_with_awareness(color: &str, input: &str, replace_with: &str, regex: &Regex) -> String {
    let chunks = split_into_chunks(input);

    let mut output = calculate_and_allocate_capacity(input);
    for chunk in chunks {
        match chunk {
            Chunk::Normal(text) => {
                let highlighted = regex.replace_all(text, |_caps: &Captures<'_>| {
                    format!("{}{}{}", color, replace_with, color::RESET)
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

pub(crate) fn highlight_with_awareness_replace_all(color: &str, input: &str, regex: &Regex, border: bool) -> String {
    let chunks = split_into_chunks(input);

    let mut output = calculate_and_allocate_capacity(input);
    for chunk in chunks {
        match chunk {
            Chunk::Normal(text) => {
                let highlighted = regex.replace_all(text, |caps: &Captures<'_>| {
                    if border {
                        format!("{} {} {}", color, &caps[0], color::RESET)
                    } else {
                        format!("{}{}{}", color, &caps[0], color::RESET)
                    }
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

pub(crate) fn highlight_with_awareness<F>(input: &str, regex: &Regex, highlight_fn: F) -> String
where
    F: Fn(&Captures) -> String,
{
    let chunks = split_into_chunks(input);
    let mut output = calculate_and_allocate_capacity(input);

    for chunk in chunks {
        match chunk {
            Chunk::Normal(text) => {
                let highlighted = regex.replace_all(text, |caps: &Captures<'_>| highlight_fn(caps));
                output.push_str(&highlighted);
            }
            Chunk::Highlighted(text) => {
                output.push_str(text);
            }
        }
    }

    output
}

fn calculate_and_allocate_capacity(input: &str) -> String {
    let allocation_size = input.len().saturating_mul(3);
    let allocation_size = std::cmp::min(allocation_size, MAX_ALLOCATION_SIZE);

    String::with_capacity(allocation_size)
}

enum Chunk<'a> {
    Normal(&'a str),
    Highlighted(&'a str),
}

fn split_into_chunks(input: &str) -> Vec<Chunk> {
    let reset_code = "\x1b[0m";
    let escape_code = "\x1b[";

    let mut chunks = Vec::new();
    let mut start = 0;
    let mut inside_escape = false;

    while let Some(i) = if inside_escape {
        input[start..].find(reset_code)
    } else {
        input[start..].find(escape_code)
    } {
        let i = i + start;
        if inside_escape {
            chunks.push(Chunk::Highlighted(&input[start..=i + reset_code.len() - 1]));
            start = i + reset_code.len();
        } else {
            if i != start {
                chunks.push(Chunk::Normal(&input[start..i]));
            }
            start = i;
        }
        inside_escape = !inside_escape;
    }

    if start != input.len() {
        chunks.push(if inside_escape {
            Chunk::Highlighted(&input[start..])
        } else {
            Chunk::Normal(&input[start..])
        });
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
        let result = highlight_with_awareness_replace_all(color, input, &regex, false);

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
        let result = highlight_with_awareness_replace_all(color, input, &regex, false);

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
