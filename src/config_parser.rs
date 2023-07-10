use crate::color::{Bg, Fg};

use serde::Deserialize;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

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
pub struct Uuid {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Ip {
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
pub struct Url {
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
    pub uuid: Option<Uuid>,
    pub url: Option<Url>,
    pub ip: Option<Ip>,
    pub path: Option<FilePath>,
    pub keywords: Option<Vec<Keyword>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub groups: Groups,
}

pub fn load_config(path: Option<String>) -> Config {
    // Obtain the home directory
    let home_dir = env::var("HOME").expect("HOME directory not set");
    let home_path = PathBuf::from(home_dir);

    // Construct the path to the default configuration file
    let default_config_path = home_path.join(".config/tailspin/config.toml");

    let path = path.or_else(|| {
        // If no path is provided, and if a config exists at the default path, use it
        if default_config_path.exists() {
            Some(
                default_config_path
                    .to_str()
                    .expect("Invalid path")
                    .to_owned(),
            )
        } else {
            // If no path is provided and no config exists at the default path, use default
            None
        }
    });

    match path {
        Some(path) => {
            let p = &PathBuf::from(path);
            let contents = fs::read_to_string(p).expect("Could not read file");

            match toml::from_str::<Config>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    println!("Could not deserialize file:\n\n{}", err);
                    exit(1);
                }
            }
        }
        // If no file was found, use the default configuration
        None => match toml::from_str(DEFAULT_CONFIG) {
            Ok(config) => config,
            Err(err) => {
                println!("Could not deserialize default config:\n\n{}", err);
                exit(1);
            }
        },
    }
}
