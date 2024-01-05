pub mod defaults;

use crate::color::{Bg, Fg};

use serde::Deserialize;

#[derive(Eq, PartialEq, Hash, Debug, Deserialize, Default, Clone)]
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
    pub style: Style,
    pub shorten: Option<Shorten>,
    pub disabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Shorten {
    pub to: String,
    pub style: Style,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Time {
    pub time: Style,
    pub zone: Style,
    pub shorten: Option<Shorten>,
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

#[derive(Eq, Hash, PartialEq, Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Keyword {
    pub style: Style,
    pub words: Vec<String>,
    pub border: bool,
}

#[derive(Eq, Hash, PartialEq, Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Regexp {
    pub regular_expression: String,
    pub style: Style,
    pub border: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Theme {
    #[serde(default)]
    pub date: Date,
    #[serde(default)]
    pub time: Time,
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
    #[serde(default)]
    pub keywords: Option<Vec<Keyword>>,
    #[serde(default)]
    pub regexps: Option<Vec<Regexp>>,
}
