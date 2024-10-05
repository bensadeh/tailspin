use crate::theme_legacy::raw::Ip;
use inlet_manifold::{NumberConfig, Style, UuidConfig};
use serde::Deserialize;

mod mappers;
pub mod reader;

pub struct Theme {
    pub numbers: NumberConfig,
    pub uuids: UuidConfig,
    // pub quotes: QuotesConfig,
    // pub ip_addresses: IpConfig,
    // pub dates: DateConfig,
    // pub paths: PathConfig,
    // pub urls: UrlConfig,
    // pub pointers: PointerConfig,
    // pub processes: ProcessConfig,
    // pub key_value_pairs: KeyValueConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct TomlTheme {
    pub numbers: Option<NumberToml>,
    pub uuids: Option<UuidToml>,
    pub quotes: Option<QuotesToml>,
    pub ip_addresses: Option<IpToml>,
    pub dates: Option<DateToml>,
    pub paths: Option<PathToml>,
    pub urls: Option<UrlToml>,
    pub pointers: Option<PointerToml>,
    pub processes: Option<ProcessToml>,
    pub key_value_pairs: Option<KeyValueToml>,
}

#[derive(Deserialize, Debug)]
pub struct NumberToml {
    pub style: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct UuidToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub dash: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct QuotesToml {
    pub quote: Style,
    pub double_quote: Style,
}

#[derive(Deserialize, Debug)]
pub struct IpToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub separator: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct DateToml {
    pub style: Style,
}

#[derive(Deserialize, Debug)]
pub struct PathToml {
    pub segment: Style,
    pub separator: Style,
}

#[derive(Deserialize, Debug)]
pub struct UrlToml {
    pub style: Style,
}

#[derive(Deserialize, Debug)]
pub struct PointerToml {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
    pub separator_token: char,
    pub x: Style,
}

#[derive(Deserialize, Debug)]
pub struct ProcessToml {
    pub name: Style,
    pub separator: Style,
    pub id: Style,
}

#[derive(Deserialize, Debug)]
pub struct KeyValueToml {
    pub key: Style,
    pub separator: Style,
}
