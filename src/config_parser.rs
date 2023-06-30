use crate::color::{Bg, Fg};

use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::exit;

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
pub struct Keyword {
    pub style: Style,
    pub words: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct UUID {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct IP {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FilePath {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Date {
    pub style: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Number {
    pub style: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Quotes {
    pub style: Style,
    #[serde(default = "default_quotes_token")]
    pub(crate) token: char,
}

fn default_quotes_token() -> char {
    '"'
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct URL {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Groups {
    pub date: Option<Date>,
    pub number: Option<Number>,
    pub quotes: Option<Quotes>,
    pub uuid: Option<UUID>,
    pub url: Option<URL>,
    pub ip: Option<IP>,
    pub path: Option<FilePath>,
    pub keywords: Option<Vec<Keyword>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub groups: Groups,
}

pub fn load_config(path: Option<String>) -> Config {
    match path {
        Some(path) => {
            let p = &Path::new(&path);
            let contents = fs::read_to_string(p).expect("Could not read file");

            match toml::from_str::<Config>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    println!("Could not deserialize file:\n\n{}", err);
                    exit(1);
                }
            }
        }
        None => match toml::from_str(DEFAULT_CONFIG) {
            Ok(config) => config,
            Err(err) => {
                println!("Could not deserialize default config:\n\n{}", err);
                exit(1);
            }
        },
    }
}
