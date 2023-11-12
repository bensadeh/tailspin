use crate::theme::{Keyword, Style};
use std::collections::HashMap;

pub fn consolidate_keywords(keywords: Vec<Keyword>) -> Vec<Keyword> {
    let mut map: HashMap<(Style, bool), Vec<String>> = HashMap::new();

    for keyword in keywords {
        map.entry((keyword.style, keyword.border))
            .or_default()
            .extend(keyword.words);
    }

    map.into_iter()
        .map(|((style, border), words)| Keyword { style, words, border })
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

        let keywords = vec![
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

        let consolidated = consolidate_keywords(keywords);

        let expected = vec![
            Keyword {
                style: style_red.clone(),
                words: vec!["apple", "banana", "orange"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                border: true,
            },
            Keyword {
                style: style_red.clone(),
                words: vec!["melon"].into_iter().map(String::from).collect(),
                border: false,
            },
            Keyword {
                style: style_default.clone(),
                words: vec!["grape"].into_iter().map(String::from).collect(),
                border: false,
            },
        ];

        assert_eq!(consolidated.len(), expected.len());
        for expected_keyword in expected {
            assert!(consolidated.contains(&expected_keyword));
        }
    }
}
