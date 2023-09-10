use crate::color::{Bg, Fg};

use serde::Deserialize;

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
pub struct KeyValue {
    pub key: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FilePath {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Date {
    pub date: Style,
    pub time: Style,
    pub zone: Style,
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
    pub key_value: Option<KeyValue>,
    pub path: Option<FilePath>,
    pub keywords: Option<Vec<Keyword>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Theme {
    pub groups: Groups,
}
