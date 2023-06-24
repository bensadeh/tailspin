use crate::color;
use regex::{Captures, Regex};

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
    let escape_code_regex = Regex::new(r"\x1b\[\d+m").unwrap();
    let reset_code = "\x1b[0m";

    let mut rest = input;
    let mut inside_escape = false;
    let mut chunks = Vec::new();

    while !rest.is_empty() {
        if !inside_escape {
            if let Some(mat) = escape_code_regex.find(rest) {
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
