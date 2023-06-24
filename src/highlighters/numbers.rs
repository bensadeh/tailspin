use crate::color;
use crate::color::to_ansi;
use crate::config_parser::Style;
use crate::highlighters::HighlightFn;
use regex::Regex;

pub fn highlight(style: &Style) -> HighlightFn {
    let color = to_ansi(style);

    Box::new(move |input: &str| -> String { highlight_numbers(&color, input) })
}

fn highlight_numbers(color: &str, input: &str) -> String {
    let number_regex = Regex::new(r"\b\d+\b").expect("Invalid regex pattern");

    highlight_with_awareness(color, input, |color, text| {
        number_regex
            .replace_all(text, |caps: &regex::Captures<'_>| {
                format!("{}{}{}", color, &caps[0], color::RESET)
            })
            .into_owned()
    })
}

fn highlight_with_awareness<F>(color: &str, input: &str, highlighter: F) -> String
where
    F: Fn(&str, &str) -> String,
{
    let chunks = split_into_chunks(input);

    let mut output = String::new();
    for chunk in chunks {
        match chunk {
            Chunk::Normal(text) => {
                let highlighted = highlighter(color, text);
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
