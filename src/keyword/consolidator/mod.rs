use crate::theme::processed::Keyword;
use std::collections::HashSet;

pub fn consolidate_keywords(keywords: Vec<Keyword>) -> Vec<Keyword> {
    let mut consolidated: Vec<Keyword> = Vec::new();

    for keyword in keywords {
        let mut found = false;
        for cons in consolidated.iter_mut() {
            if cons.style == keyword.style && cons.border == keyword.border {
                let mut words_set: HashSet<String> = cons.words.iter().cloned().collect();
                words_set.extend(keyword.words.clone());
                cons.words = words_set.into_iter().collect();
                found = true;
                break;
            }
        }
        if !found {
            consolidated.push(keyword);
        }
    }

    consolidated
        .into_iter()
        .filter(|keyword| !keyword.words.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nu_ansi_term::{Color, Style};

    #[test]
    fn test_consolidate_keywords() {
        let input_keywords = vec![
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["apple".into(), "banana".into()],
                border: true,
            },
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["orange".into()],
                border: true,
            },
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["melon".into()],
                border: false,
            },
            Keyword {
                style: Style::default(),
                words: vec!["grape".into()],
                border: false,
            },
        ];

        let actual = consolidate_keywords(input_keywords);

        let expected = vec![
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["apple".into(), "banana".into(), "orange".into()],
                border: true,
            },
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["melon".into()],
                border: false,
            },
            Keyword {
                style: Style::default(),
                words: vec!["grape".into()],
                border: false,
            },
        ];

        assert_eq!(actual.len(), expected.len());
    }

    #[test]
    fn test_different_styles_and_borders() {
        let keywords = vec![
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["apple".into(), "banana".into()],
                border: true,
            },
            Keyword {
                style: Style::new().fg(Color::Blue),
                words: vec!["orange".into()],
                border: false,
            },
        ];

        let consolidated = consolidate_keywords(keywords);
        assert_eq!(consolidated.len(), 2);
    }

    #[test]
    fn test_same_name_and_different_styles() {
        let keywords = vec![
            Keyword {
                style: Style::new().fg(Color::Green),
                words: vec!["apple".into()],
                border: true,
            },
            Keyword {
                style: Style::new().fg(Color::Yellow),
                words: vec!["apple".into()],
                border: false,
            },
        ];

        let consolidated = consolidate_keywords(keywords);
        assert_eq!(consolidated.len(), 2);
    }

    #[test]
    fn test_duplicate_words() {
        let keywords = vec![
            Keyword {
                style: Style::new().fg(Color::Red),
                words: vec!["apple".into(), "banana".into()],
                border: true,
            },
            Keyword {
                style: Style::new().fg(Color::Red),
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
            style: Style::new().fg(Color::Red),
            words: vec![],
            border: true,
        }];

        let consolidated = consolidate_keywords(keywords);
        assert!(consolidated.is_empty());
    }

    #[test]
    fn test_single_keyword() {
        let keywords = vec![Keyword {
            style: Style::new().fg(Color::Red),
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
