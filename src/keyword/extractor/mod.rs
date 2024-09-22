use crate::theme_legacy::processed::Keyword;
use nu_ansi_term::{Color, Style};

pub fn extract_all_keywords(
    words_red: Vec<String>,
    words_green: Vec<String>,
    words_yellow: Vec<String>,
    words_blue: Vec<String>,
    words_magenta: Vec<String>,
    words_cyan: Vec<String>,
) -> Vec<Keyword> {
    [
        extract_keywords(words_red, Color::Red),
        extract_keywords(words_green, Color::Green),
        extract_keywords(words_yellow, Color::Yellow),
        extract_keywords(words_blue, Color::Blue),
        extract_keywords(words_magenta, Color::Magenta),
        extract_keywords(words_cyan, Color::Cyan),
    ]
    .concat()
}

pub fn extract_keywords(words: Vec<String>, color: Color) -> Vec<Keyword> {
    words
        .into_iter()
        .map(|word| Keyword {
            style: Style::from(color),
            words: vec![word],
            ..Default::default()
        })
        .collect()
}
