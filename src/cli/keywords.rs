use crate::cli::Arguments;
use tailspin::{Color, KeywordConfig, Style};

pub fn get_keywords_from_cli(cli: &Arguments) -> Vec<KeywordConfig> {
    vec![]
        .into_iter()
        .chain(extract_keywords(&cli.words_red, Color::Red))
        .chain(extract_keywords(&cli.words_green, Color::Green))
        .chain(extract_keywords(&cli.words_yellow, Color::Yellow))
        .chain(extract_keywords(&cli.words_blue, Color::Blue))
        .chain(extract_keywords(&cli.words_magenta, Color::Magenta))
        .chain(extract_keywords(&cli.words_cyan, Color::Cyan))
        .collect()
}

fn extract_keywords(words: &[String], color: Color) -> Vec<KeywordConfig> {
    words
        .iter()
        .map(|word| KeywordConfig {
            style: Style::new().fg(color),
            words: vec![word.clone()],
        })
        .collect()
}
