use crate::config_parser::{Keyword, Style};

#[derive(Debug, Clone)]
pub struct FlattenKeyword {
    pub keyword: String,
    pub style: Style,
}

pub fn flatten_keywords(keywords: Vec<Keyword>) -> Vec<FlattenKeyword> {
    let mut flatten_keywords = Vec::new();

    for keyword in keywords {
        for string in keyword.words {
            flatten_keywords.push(FlattenKeyword {
                keyword: string,
                style: keyword.style.clone(),
            });
        }
    }

    flatten_keywords
}
