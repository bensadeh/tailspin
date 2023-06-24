use crate::color::{Bg, Fg};

use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../data/config.toml");

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Style {
    #[serde(default)]
    pub fg: Fg,
    #[serde(default)]
    pub bg: Bg,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub faint: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct KeywordGroup {
    pub highlight: Style,
    pub tokens: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct UuidGroup {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Settings {
    #[serde(default = "default_quotes_token")]
    pub(crate) quotes_token: char,
}

fn default_quotes_token() -> char {
    '"'
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Groups {
    pub dates: Option<Style>,
    pub numbers: Option<Style>,
    pub quotes: Option<Style>,
    pub uuids: Option<UuidGroup>,
    pub keywords: Option<Vec<KeywordGroup>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub settings: Settings,
    pub groups: Groups,
}

pub fn load_config(path: Option<String>) -> Config {
    match path {
        Some(path) => {
            let p = &Path::new(&path);
            let contents = fs::read_to_string(p).expect("Could not read file");

            toml::from_str(&contents).expect("Could not deserialize file")
        }
        None => toml::from_str(DEFAULT_CONFIG).expect("Could not deserialize default config"),
    }
}
