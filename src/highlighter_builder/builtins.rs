use tailspin::config::KeywordConfig;
use tailspin::{Color, Style};

pub fn get_builtin_keywords(disable_builtin_keywords: bool) -> Vec<KeywordConfig> {
    match disable_builtin_keywords {
        true => vec![],
        false => builtin_keywords(),
    }
}

fn builtin_keywords() -> Vec<KeywordConfig> {
    let severity_levels = vec![
        KeywordConfig {
            words: vec!["ERROR".to_string()],
            style: Style::new().fg(Color::Red),
        },
        KeywordConfig {
            words: vec!["WARN".to_string(), "WARNING".to_string()],
            style: Style::new().fg(Color::Yellow),
        },
        KeywordConfig {
            words: vec!["INFO".to_string()],
            style: Style::new().fg(Color::White),
        },
        KeywordConfig {
            words: vec!["SUCCESS".to_string(), "DEBUG".to_string()],
            style: Style::new().fg(Color::Green),
        },
        KeywordConfig {
            words: vec!["TRACE".to_string()],
            style: Style::new().faint(),
        },
    ];

    let rest_keywords = vec![
        KeywordConfig {
            words: vec!["GET".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Green),
        },
        KeywordConfig {
            words: vec!["POST".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Yellow),
        },
        KeywordConfig {
            words: vec!["PUT".to_string(), "PATCH".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Magenta),
        },
        KeywordConfig {
            words: vec!["DELETE".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Red),
        },
    ];

    let booleans = [KeywordConfig {
        words: vec!["null".to_string(), "true".to_string(), "false".to_string()],
        style: Style::new().fg(Color::Red).italic(),
    }];

    vec![]
        .into_iter()
        .chain(severity_levels)
        .chain(rest_keywords)
        .chain(booleans)
        .collect()
}
