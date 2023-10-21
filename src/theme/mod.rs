mod defaults;

use crate::color::{Bg, Fg};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
//remove this
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
    #[serde(default)]
    pub border: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Uuid {
    pub segment: Style,
    pub separator: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Ip {
    pub segment: Style,
    pub separator: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct KeyValue {
    pub key: Style,
    pub separator: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct FilePath {
    pub segment: Style,
    pub separator: Style,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Date {
    pub date: Style,
    pub time: Style,
    pub zone: Style,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Process {
    pub name: Style,
    pub id: Style,
    pub separator: Style,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Number {
    pub style: Style,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Quotes {
    pub style: Style,
    #[serde(default = "default_quotes_token")]
    pub(crate) token: char,
    #[serde(default)]
    pub disabled: bool,
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
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Groups {
    pub date: Option<Date>,
    pub number: Option<Number>,
    pub quotes: Option<Quotes>,
    #[serde(default)]
    pub uuid: Uuid,
    pub url: Option<Url>,
    #[serde(default)]
    pub ip: Ip,
    #[serde(default)]
    pub key_value: KeyValue,
    pub path: Option<FilePath>,
    pub process: Option<Process>,
    pub keywords: Option<Vec<Keyword>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Theme {
    pub groups: Groups,
}
