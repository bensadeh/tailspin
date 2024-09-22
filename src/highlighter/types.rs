use inlet_manifold::*;
use serde::Deserialize;

pub struct HighlighterConfig {
    pub uuid: bool,
}

pub struct ProcessedTheme {
    pub uuid_config: UuidConfig,
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
