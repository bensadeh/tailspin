use crate::colors::{Bg, Fg, Style};
use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../data/config.toml");

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Highlight {
    #[serde(default)]
    pub fg: Fg,
    #[serde(default)]
    pub bg: Bg,
    #[serde(default)]
    pub style: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct KeywordGroup {
    pub highlight: Highlight,
    pub tokens: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Settings {
    #[serde(default = "default_quotes_token")]
    quotes_token: String,
}

fn default_quotes_token() -> String {
    String::from("\"")
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Groups {
    pub numbers: Option<Highlight>,
    pub quotes: Option<Highlight>,
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
