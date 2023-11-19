use crate::color::Fg;
use crate::theme::{Keyword, Style};

pub fn extract_all_keywords(
    words_red: Vec<String>,
    words_green: Vec<String>,
    words_yellow: Vec<String>,
    words_blue: Vec<String>,
    words_magenta: Vec<String>,
    words_cyan: Vec<String>,
) -> Vec<Keyword> {
    [
        extract_keywords(words_red, Fg::Red),
        extract_keywords(words_green, Fg::Green),
        extract_keywords(words_yellow, Fg::Yellow),
        extract_keywords(words_blue, Fg::Blue),
        extract_keywords(words_magenta, Fg::Magenta),
        extract_keywords(words_cyan, Fg::Cyan),
    ]
    .concat()
}

pub fn extract_keywords(words: Vec<String>, color: Fg) -> Vec<Keyword> {
    words
        .into_iter()
        .map(|word| Keyword {
            style: Style {
                fg: color,
                ..Default::default()
            },
            words: vec![word],
            ..Default::default()
        })
        .collect()
}
