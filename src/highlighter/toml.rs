use inlet_manifold::*;
use serde::Deserialize;

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
