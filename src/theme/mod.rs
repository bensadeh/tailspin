pub mod defaults;

use crate::color::{Bg, Fg};

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Style {
    pub fg: Fg,
    pub bg: Bg,
    pub bold: bool,
    pub faint: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Keyword {
    pub style: Style,
    pub words: Vec<String>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct FilePath {
    pub segment: Style,
    pub separator: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Date {
    pub date: Style,
    pub time: Style,
    pub zone: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Process {
    pub name: Style,
    pub id: Style,
    pub separator: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Number {
    pub style: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Quotes {
    pub style: Style,
    pub token: char,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Url {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Groups {
    #[serde(default)]
    pub date: Date,
    #[serde(default)]
    pub number: Number,
    #[serde(default)]
    pub quotes: Quotes,
    #[serde(default)]
    pub uuid: Uuid,
    #[serde(default)]
    pub url: Url,
    #[serde(default)]
    pub ip: Ip,
    #[serde(default)]
    pub key_value: KeyValue,
    #[serde(default)]
    pub path: FilePath,
    #[serde(default)]
    pub process: Process,
    pub keywords: Option<Vec<Keyword>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Theme {
    pub groups: Groups,
}
