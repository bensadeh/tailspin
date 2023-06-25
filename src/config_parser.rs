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
pub struct IpGroup {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct UrlGroup {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
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
    pub date: Option<Style>,
    pub number: Option<Style>,
    pub quotes: Option<Style>,
    pub uuid: Option<UuidGroup>,
    pub url: Option<UrlGroup>,
    pub ip: Option<IpGroup>,
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
