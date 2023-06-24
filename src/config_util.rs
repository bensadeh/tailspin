use crate::config_parser::{KeywordGroup, Style};

#[derive(Debug, Clone)]
pub struct FlattenKeyword {
    pub keyword: String,
    pub style: Style,
}

pub fn flatten_keywords(keywords: Vec<KeywordGroup>) -> Vec<FlattenKeyword> {
    let mut flatten_keywords = Vec::new();

    for keyword in keywords {
        for string in keyword.tokens {
            flatten_keywords.push(FlattenKeyword {
                keyword: string,
                style: keyword.highlight.clone(),
            });
        }
    }

    flatten_keywords
}
