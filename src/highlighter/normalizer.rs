use crate::{KeywordConfig, Style};
use std::collections::HashMap;

pub fn normalize_keyword_configs(configs: Vec<KeywordConfig>) -> Vec<KeywordConfig> {
    let mut grouped_configs: HashMap<Style, Vec<String>> = HashMap::new();

    for config in configs {
        grouped_configs.entry(config.style).or_default().extend(config.words);
    }

    let mut result: Vec<KeywordConfig> = grouped_configs
        .into_iter()
        .map(|(style, words)| {
            let mut sorted_words = words.clone();
            sorted_words.sort();
            KeywordConfig {
                words: sorted_words,
                style,
            }
        })
        .collect();

    result.sort_by(|a, b| a.style.cmp(&b.style));

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color::*;
    use std::default::Default;

    #[test]
    fn test_normalize_keyword_configs() {
        let configs = vec![
            KeywordConfig {
                words: vec!["hello".to_string(), "world".to_string()],
                style: Style {
                    fg: Some(Red),
                    bold: true,
                    ..Style::default()
                },
            },
            KeywordConfig {
                words: vec!["foo".to_string(), "bar".to_string()],
                style: Style {
                    fg: Some(Red),
                    bold: true,
                    ..Style::default()
                },
            },
            KeywordConfig {
                words: vec!["baz".to_string()],
                style: Style {
                    fg: Some(Green),
                    underline: true,
                    ..Style::default()
                },
            },
        ];

        let expected = vec![
            KeywordConfig {
                words: vec![
                    "bar".to_string(),
                    "foo".to_string(),
                    "hello".to_string(),
                    "world".to_string(),
                ],
                style: Style {
                    fg: Some(Red),
                    bold: true,
                    ..Style::default()
                },
            },
            KeywordConfig {
                words: vec!["baz".to_string()],
                style: Style {
                    fg: Some(Green),
                    underline: true,
                    ..Style::default()
                },
            },
        ];

        let actual = normalize_keyword_configs(configs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_normalize_keyword_configs_empty() {
        let configs: Vec<KeywordConfig> = vec![];
        let expected: Vec<KeywordConfig> = vec![];
        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }

    #[test]
    fn test_normalize_keyword_simple_grouping() {
        let configs = vec![
            KeywordConfig {
                words: vec!["error".to_string()],
                style: Style::new().fg(Red),
            },
            KeywordConfig {
                words: vec!["null".to_string()],
                style: Style::new().fg(Red),
            },
        ];

        let expected = vec![KeywordConfig {
            words: vec!["error".to_string(), "null".to_string()],
            style: Style::new().fg(Red),
        }];

        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }

    #[test]
    fn test_do_not_normalize_slightly_different_groupings() {
        let configs = vec![
            KeywordConfig {
                words: vec!["error".to_string()],
                style: Style {
                    fg: Some(Red),
                    bold: true,
                    ..Style::default()
                },
            },
            KeywordConfig {
                words: vec!["null".to_string()],
                style: Style {
                    fg: Some(Red),
                    italic: true,
                    ..Style::default()
                },
            },
        ];

        let expected = vec![
            KeywordConfig {
                words: vec!["null".to_string()],
                style: Style {
                    fg: Some(Red),
                    italic: true,
                    ..Style::default()
                },
            },
            KeywordConfig {
                words: vec!["error".to_string()],
                style: Style {
                    fg: Some(Red),
                    bold: true,
                    ..Style::default()
                },
            },
        ];

        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }

    #[test]
    fn test_normalize_keyword_configs_no_duplicates() {
        let configs = vec![KeywordConfig {
            words: vec!["unique".to_string()],
            style: Style {
                fg: Some(Blue),
                italic: true,
                ..Style::default()
            },
        }];

        let expected = vec![KeywordConfig {
            words: vec!["unique".to_string()],
            style: Style {
                fg: Some(Blue),
                italic: true,
                ..Style::default()
            },
        }];

        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }
}
