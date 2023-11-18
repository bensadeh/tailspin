use crate::theme::{Keyword, Style};
use std::collections::{HashMap, HashSet};

pub fn consolidate_keywords(keywords: Vec<Keyword>) -> Vec<Keyword> {
    let mut map: HashMap<(Style, bool), HashSet<String>> = HashMap::new();

    for keyword in keywords {
        map.entry((keyword.style, keyword.border))
            .or_default()
            .extend(keyword.words.into_iter());
    }

    map.into_iter()
        .map(|((style, border), words)| Keyword {
            style,
            words: words.into_iter().collect(),
            border,
        })
        .filter(|keyword| !keyword.words.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Fg;

    #[test]
    fn test_consolidate_keywords() {
        let style_red = Style {
            fg: Fg::Red,
            ..Default::default()
        };

        let style_default = Style { ..Default::default() };

        let input_keywords = vec![
            Keyword {
                style: style_red.clone(),
                words: vec!["apple".into(), "banana".into()],
                border: true,
            },
            Keyword {
                style: style_red.clone(),
                words: vec!["orange".into()],
                border: true,
            },
            Keyword {
                style: style_red.clone(),
                words: vec!["melon".into()],
                border: false,
            },
            Keyword {
                style: style_default.clone(),
                words: vec!["grape".into()],
                border: false,
            },
        ];

        let actual = consolidate_keywords(input_keywords);

        let expected = vec![
            Keyword {
                style: style_red.clone(),
                words: vec!["apple".into(), "banana".into(), "orange".into()],
                border: true,
            },
            Keyword {
                style: style_red.clone(),
                words: vec!["melon".into()],
                border: false,
            },
            Keyword {
                style: style_default.clone(),
                words: vec!["grape".into()],
                border: false,
            },
        ];

        assert_eq!(actual.len(), expected.len());
    }

    #[test]
    fn test_different_styles_and_borders() {
        let style_one = Style {
            fg: Fg::Red,
            ..Default::default()
        };
        let style_two = Style {
            fg: Fg::Blue,
            ..Default::default()
        };

        let keywords = vec![
            Keyword {
                style: style_one,
                words: vec!["apple".into(), "banana".into()],
                border: true,
            },
            Keyword {
                style: style_two,
                words: vec!["orange".into()],
                border: false,
            },
        ];

        let consolidated = consolidate_keywords(keywords);
        assert_eq!(consolidated.len(), 2);
    }

    #[test]
    fn test_same_name_and_different_styles() {
        let style_one = Style {
            fg: Fg::Green,
            ..Default::default()
        };
        let style_two = Style {
            fg: Fg::Yellow,
            ..Default::default()
        };

        let keywords = vec![
            Keyword {
                style: style_one,
                words: vec!["apple".into()],
                border: true,
            },
            Keyword {
                style: style_two,
                words: vec!["apple".into()],
                border: false,
            },
        ];

        let consolidated = consolidate_keywords(keywords);
        assert_eq!(consolidated.len(), 2);
    }

    #[test]
    fn test_duplicate_words() {
        let style = Style {
            fg: Fg::Red,
            ..Default::default()
        };

        let keywords = vec![
            Keyword {
                style: style.clone(),
                words: vec!["apple".into(), "banana".into()],
                border: true,
            },
            Keyword {
                style,
                words: vec!["banana".into(), "cherry".into()],
                border: true,
            },
        ];

        let consolidated = consolidate_keywords(keywords);
        assert_eq!(consolidated.len(), 1);
        assert!(consolidated[0].words.contains(&"banana".to_string()));
        assert_eq!(consolidated[0].words.len(), 3);
    }

    #[test]
    fn test_empty_words_list() {
        let keywords = vec![Keyword {
            style: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            words: vec![],
            border: true,
        }];

        let consolidated = consolidate_keywords(keywords);
        assert!(consolidated.is_empty());
    }

    #[test]
    fn test_single_keyword() {
        let style = Style {
            fg: Fg::Red,
            ..Default::default()
        };

        let keywords = vec![Keyword {
            style,
            words: vec!["apple".into()],
            border: true,
        }];

        let consolidated = consolidate_keywords(keywords);
        assert_eq!(consolidated.len(), 1);
        assert_eq!(consolidated[0].words, vec!["apple".to_string()]);
    }

    #[test]
    fn test_no_keywords() {
        let keywords: Vec<Keyword> = vec![];

        let consolidated = consolidate_keywords(keywords);
        assert!(consolidated.is_empty());
    }
}
