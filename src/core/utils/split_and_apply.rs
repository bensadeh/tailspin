use crate::core::highlighter::Highlight;
use crate::core::highlighters::StaticHighlight;
use std::borrow::Cow;
use std::cmp::min;

const FOUR_KB: usize = 4 * 1024; // 4 KiB

#[derive(Default)]
pub struct FoldState<'a> {
    /// Holds the new string once a change is detected.
    result: Option<String>,
    /// The number of bytes processed without change.
    processed: usize,
    /// The original input, used for copying the unchanged prefix.
    input: &'a str,
}

impl FoldState<'_> {
    fn update(&mut self, text: &str) {
        match self.result {
            Some(ref mut buf) => buf.push_str(text),
            None => self.processed += text.len(),
        }
    }

    fn update_owned(&mut self, new_text: &str) {
        match self.result {
            None => {
                let mut buf = allocate_string(self.input);
                buf.push_str(&self.input[..self.processed]);
                buf.push_str(new_text);
                self.result = Some(buf);
            }
            Some(_) => self.result.as_mut().unwrap().push_str(new_text),
        }
    }
}

pub fn apply_only_to_unhighlighted<'a>(input: &'a str, highlighter: &StaticHighlight) -> Cow<'a, str> {
    let initial_state = FoldState {
        input,
        ..FoldState::default()
    };

    split_into_chunks(input)
        .iter()
        .fold(initial_state, |mut state, chunk| {
            match chunk {
                Chunk::AlreadyHighlighted(text) => state.update(text),
                Chunk::NotHighlighted(text) => {
                    let transformed = highlighter.apply(text);
                    match transformed {
                        Cow::Borrowed(new_text) => state.update(new_text),
                        Cow::Owned(new_text) => state.update_owned(&new_text),
                    }
                }
            }
            state
        })
        .result
        .map_or(Cow::Borrowed(input), Cow::Owned)
}

fn allocate_string(input: &str) -> String {
    let input_length_times_3 = input.len().saturating_mul(3);
    let allocation_size = min(input_length_times_3, FOUR_KB);

    String::with_capacity(allocation_size)
}

enum Chunk<'a> {
    NotHighlighted(&'a str),
    AlreadyHighlighted(&'a str),
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
            chunks.push(Chunk::AlreadyHighlighted(&input[start..=i + reset_code.len() - 1]));
            start = i + reset_code.len();
        } else {
            if i != start {
                chunks.push(Chunk::NotHighlighted(&input[start..i]));
            }
            start = i;
        }
        inside_escape = !inside_escape;
    }

    if start != input.len() {
        chunks.push(if inside_escape {
            Chunk::AlreadyHighlighted(&input[start..])
        } else {
            Chunk::NotHighlighted(&input[start..])
        });
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_chunks() {
        let input = "Here is a date \x1b[31m2023-06-24\x1b[0m, and here is a number 12345.";
        let chunks = split_into_chunks(input);

        assert_eq!(chunks.len(), 3);
        match &chunks[0] {
            Chunk::NotHighlighted(text) => assert_eq!(*text, "Here is a date "),
            _ => panic!("Unexpected chunk type."),
        }
        match &chunks[1] {
            Chunk::AlreadyHighlighted(text) => assert_eq!(*text, "\x1b[31m2023-06-24\x1b[0m"),
            _ => panic!("Unexpected chunk type."),
        }
        match &chunks[2] {
            Chunk::NotHighlighted(text) => assert_eq!(*text, ", and here is a number 12345."),
            _ => panic!("Unexpected chunk type."),
        }
    }
}
