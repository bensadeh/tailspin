use inlet_manifold::{NumberConfig, Style, UuidConfig};
use serde::Deserialize;

mod mappers;

pub struct Theme {
    pub uuid: UuidConfig,
    pub number: NumberConfig,
}

#[derive(Deserialize, Debug)]
pub struct TomlTheme {
    pub uuid: Option<UuidToml>,
    pub number: Option<NumberToml>,
}

#[derive(Deserialize, Debug)]
pub struct UuidToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub dash: Option<Style>,
}

#[derive(Deserialize, Debug)]
pub struct NumberToml {
    pub style: Option<Style>,
}
