use crate::Style;

pub struct NumberConfig {
    pub style: Style,
}

pub struct UuidConfig {
    pub number: Style,
    pub letter: Style,
    pub dash: Style,
}

pub struct KeyValueConfig {
    pub key: Style,
    pub separator: Style,
}

#[derive(Clone, Copy)]
pub struct DateTimeConfig {
    pub date: Style,
    pub time: Style,
    pub zone: Style,
    pub separator: Style,
}

pub struct IpV4Config {
    pub number: Style,
    pub separator: Style,
}

pub struct IpV6Config {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
}

pub struct UrlConfig {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
}

pub struct UnixPathConfig {
    pub segment: Style,
    pub separator: Style,
}

pub struct PointerConfig {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
    pub separator_token: char,
    pub x: Style,
}

pub struct UnixProcessConfig {
    pub name: Style,
    pub id: Style,
    pub bracket: Style,
}

pub struct JsonConfig {
    pub key: Style,
    pub quote_token: Style,
    pub curly_bracket: Style,
    pub square_bracket: Style,
    pub comma: Style,
    pub colon: Style,
}

pub struct QuotesConfig {
    pub quotes_token: char,
    pub style: Style,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
pub struct KeywordConfig {
    pub words: Vec<String>,
    pub style: Style,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
pub struct RegexConfig {
    pub regex: String,
    pub style: Style,
}
