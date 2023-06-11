use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../data/config.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct Highlight {
    fg: String,
    bg: String,
    style: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feature {
    enabled: bool,
    highlight: Highlight,
    #[serde(default)]
    // `symbol` is not present for all features, so we use `serde(default)` to handle its absence
    symbol: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Builtins {
    numbers: Feature,
    quotes: Feature,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Keyword {
    pub(crate) strings: Vec<String>,
    pub(crate) highlight: Highlight,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    builtins: Builtins,
    pub(crate) keywords: Vec<Keyword>,
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
