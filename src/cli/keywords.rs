use crate::cli::{Arguments, KeywordColor};
use crate::highlighter_builder::builtins::get_builtin_keywords;
use std::collections::HashSet;
use tailspin::config::KeywordConfig;
use tailspin::style::{Color, Style};

pub fn collect_keywords(cli: &Arguments, theme_keywords: Vec<KeywordConfig>) -> Vec<KeywordConfig> {
    let builtin = get_builtin_keywords(cli.disable_builtin_keywords);
    let from_cli = get_keywords_from_cli(cli);

    dedupe_last_wins(builtin.into_iter().chain(theme_keywords).chain(from_cli).collect())
}

fn dedupe_last_wins(mut configs: Vec<KeywordConfig>) -> Vec<KeywordConfig> {
    let mut seen: HashSet<String> = HashSet::new();
    for config in configs.iter_mut().rev() {
        config.words.retain(|w| seen.insert(w.clone()));
    }
    configs.retain(|c| !c.words.is_empty());
    configs
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

#[cfg(test)]
mod tests {
    use super::*;

    fn kw(words: &[&str], style: Style) -> KeywordConfig {
        KeywordConfig {
            words: words.iter().map(|s| s.to_string()).collect(),
            style,
        }
    }

    #[test]
    fn later_definition_overrides_earlier() {
        let builtin = kw(&["GET"], Style::new().fg(Color::Black).on(Color::Green));
        let user = kw(&["GET"], Style::new().fg(Color::Green).on(Color::Black));

        let result = dedupe_last_wins(vec![builtin, user.clone()]);

        assert_eq!(result, vec![user]);
    }

    #[test]
    fn unrelated_words_in_overridden_group_survive() {
        let builtin = kw(&["GET", "POST"], Style::new().fg(Color::Black).on(Color::Green));
        let user = kw(&["GET"], Style::new().fg(Color::Green));

        let result = dedupe_last_wins(vec![builtin, user.clone()]);

        assert_eq!(
            result,
            vec![kw(&["POST"], Style::new().fg(Color::Black).on(Color::Green)), user,]
        );
    }

    #[test]
    fn cli_overrides_theme_overrides_builtin() {
        let builtin = kw(&["GET"], Style::new().fg(Color::Red));
        let theme = kw(&["GET"], Style::new().fg(Color::Yellow));
        let cli = kw(&["GET"], Style::new().fg(Color::Green));

        let result = dedupe_last_wins(vec![builtin, theme, cli.clone()]);

        assert_eq!(result, vec![cli]);
    }

    #[test]
    fn theme_internal_duplicates_use_last_wins() {
        let first = kw(&["GET"], Style::new().fg(Color::Red));
        let second = kw(&["GET"], Style::new().fg(Color::Blue));

        let result = dedupe_last_wins(vec![first, second.clone()]);

        assert_eq!(result, vec![second]);
    }

    #[test]
    fn no_duplicates_passes_through_unchanged() {
        let configs = vec![
            kw(&["GET"], Style::new().fg(Color::Red)),
            kw(&["POST"], Style::new().fg(Color::Blue)),
        ];

        let result = dedupe_last_wins(configs.clone());

        assert_eq!(result, configs);
    }

    #[test]
    fn case_sensitive_dedup() {
        let lower = kw(&["get"], Style::new().fg(Color::Red));
        let upper = kw(&["GET"], Style::new().fg(Color::Blue));

        let result = dedupe_last_wins(vec![lower.clone(), upper.clone()]);

        assert_eq!(result, vec![lower, upper]);
    }

    #[test]
    fn empty_input_returns_empty() {
        let result = dedupe_last_wins(Vec::new());
        assert!(result.is_empty());
    }
}
