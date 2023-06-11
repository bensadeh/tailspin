use crate::config_parser::{Highlight, KeywordGroup};

#[derive(Debug, Clone)]
pub struct FlattenKeyword {
    string: String,
    highlight: Highlight,
}

pub fn flatten_keywords(keywords: Vec<KeywordGroup>) -> Vec<FlattenKeyword> {
    let mut flatten_keywords = Vec::new();

    for keyword in keywords {
        for string in keyword.tokens {
            flatten_keywords.push(FlattenKeyword {
                string,
                highlight: keyword.highlight.clone(),
            });
        }
    }

    flatten_keywords
}
