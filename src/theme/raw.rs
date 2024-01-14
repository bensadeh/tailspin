use serde::Deserialize;

#[derive(Eq, PartialEq, Hash, Debug, Deserialize, Default, Clone)]
pub struct Style {
    #[serde(default)]
    pub fg: String,
    #[serde(default)]
    pub bg: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub faint: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Uuid {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub dash: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct IpV4 {
    pub segment: Option<Style>,
    pub separator: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct IpV6 {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub separator: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct KeyValue {
    pub key: Option<Style>,
    pub separator: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct FilePath {
    pub segment: Option<Style>,
    pub separator: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Date {
    pub style: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Time {
    pub time: Option<Style>,
    pub zone: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Process {
    pub name: Option<Style>,
    pub id: Option<Style>,
    pub separator: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Number {
    pub style: Option<Style>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Quotes {
    pub style: Option<Style>,
    pub token: Option<char>,
    pub disabled: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Url {
    pub http: Option<Style>,
    pub https: Option<Style>,
    pub host: Option<Style>,
    pub path: Option<Style>,
    pub query_params_key: Option<Style>,
    pub query_params_value: Option<Style>,
    pub symbols: Option<Style>,
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
    pub ip_v4: IpV4,
    #[serde(default)]
    pub ip_v6: IpV6,
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
