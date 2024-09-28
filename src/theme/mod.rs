use inlet_manifold::{Style, UuidConfig};
use serde::Deserialize;

mod mappers;

pub struct Theme {
    pub uuid: UuidConfig,
}

#[derive(Deserialize, Debug)]
pub struct TomlTheme {
    pub uuid: Option<UuidToml>,
}

#[derive(Deserialize, Debug)]
pub struct UuidToml {
    pub number: Option<Style>,
    pub letter: Option<Style>,
    pub dash: Option<Style>,
}
