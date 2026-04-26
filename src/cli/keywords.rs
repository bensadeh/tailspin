use crate::cli::{Arguments, KeywordColor};
use crate::highlighter_builder::builtins::get_builtin_keywords;
use tailspin::config::KeywordConfig;
use tailspin::style::{Color, Style};

pub fn collect_keywords(cli: &Arguments, theme_keywords: Vec<KeywordConfig>) -> Vec<KeywordConfig> {
    let builtin = get_builtin_keywords(cli.disable_builtin_keywords);
    let from_cli = get_keywords_from_cli(cli);

    builtin.into_iter().chain(theme_keywords).chain(from_cli).collect()
}

fn get_keywords_from_cli(cli: &Arguments) -> Vec<KeywordConfig> {
    cli.color_word
        .iter()
        .flat_map(|(color, words)| {
            words.iter().map(move |word| KeywordConfig {
                style: Style::new().fg(Color::from(*color)),
                words: vec![word.clone()],
            })
        })
        .collect()
}

impl From<KeywordColor> for Color {
    fn from(value: KeywordColor) -> Self {
        match value {
            KeywordColor::Red => Self::Red,
            KeywordColor::Green => Self::Green,
            KeywordColor::Yellow => Self::Yellow,
            KeywordColor::Blue => Self::Blue,
            KeywordColor::Magenta => Self::Magenta,
            KeywordColor::Cyan => Self::Cyan,
        }
    }
}
