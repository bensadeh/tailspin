use serde::Deserialize;
use tailspin::*;

mod mappers;
pub mod reader;

pub struct Theme {
    pub keywords: Vec<KeywordConfig>,
    pub regexes: Vec<RegexConfig>,
    pub numbers: NumberConfig,
    pub uuids: UuidConfig,
    pub quotes: QuotesConfig,
    pub ip_v4_addresses: IpV4Config,
    pub ip_v6_addresses: IpV6Config,
    pub dates: DateTimeConfig,
    pub paths: UnixPathConfig,
    pub urls: UrlConfig,
    pub pointers: PointerConfig,
    pub processes: UnixProcessConfig,
    pub key_value_pairs: KeyValueConfig,
    pub json: JsonConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct TomlTheme {
    pub keywords: Option<Vec<KeywordToml>>,
    pub regexes: Option<Vec<RegexToml>>,
    pub numbers: Option<NumberToml>,
    pub uuids: Option<UuidToml>,
    pub quotes: Option<QuotesToml>,
    pub ip_addresses: Option<IpToml>,
    pub dates: Option<DateToml>,
    pub paths: Option<PathToml>,
    pub urls: Option<UrlToml>,
    pub pointers: Option<PointerToml>,
    pub processes: Option<UnixProcessToml>,
    pub key_value_pairs: Option<KeyValueToml>,
    pub json: Option<JsonToml>,
}

#[derive(Deserialize, Debug)]
pub struct KeywordToml {
    pub words: Vec<String>,
    pub style: Style,
}

#[derive(Deserialize, Debug)]
pub struct RegexToml {
    pub regex: String,
    pub style: Style,
}

#[derive(Deserialize, Debug)]
pub struct NumberToml {
    pub number: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct UuidToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub dash: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct QuotesToml {
    pub quotes_token: Option<char>,
    pub style: Option<Style>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct IpToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub separator: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct DateToml {
    pub date: Option<Style>,
    pub time: Option<Style>,
    pub zone: Option<Style>,
    pub separator: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct PathToml {
    pub segment: Option<Style>,
    pub separator: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct UrlToml {
    pub http: Option<Style>,
    pub https: Option<Style>,
    pub host: Option<Style>,
    pub path: Option<Style>,
    pub query_params_key: Option<Style>,
    pub query_params_value: Option<Style>,
    pub symbols: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct PointerToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub separator: Option<Style>,
    pub separator_token: Option<char>,
    pub x: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct UnixProcessToml {
    pub name: Option<Style>,
    pub id: Option<Style>,
    pub bracket: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct KeyValueToml {
    pub key: Option<Style>,
    pub separator: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct JsonToml {
    pub key: Option<Style>,
    pub quote_token: Option<Style>,
    pub curly_bracket: Option<Style>,
    pub square_bracket: Option<Style>,
    pub comma: Option<Style>,
    pub colon: Option<Style>,
}
