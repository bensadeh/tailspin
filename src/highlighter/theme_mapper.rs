use crate::highlighter::types::*;
use inlet_manifold::*;

impl From<TomlTheme> for ProcessedTheme {
    fn from(toml: TomlTheme) -> Self {
        ProcessedTheme {
            uuid_config: toml.uuid.map_or_else(UuidConfig::default, UuidConfig::from),
        }
    }
}

impl From<UuidToml> for UuidConfig {
    fn from(uuid_toml: UuidToml) -> Self {
        let default_config = UuidConfig::default();

        UuidConfig {
            number: uuid_toml.number.unwrap_or(default_config.number),
            letter: uuid_toml.letter.unwrap_or(default_config.letter),
            dash: uuid_toml.dash.unwrap_or(default_config.dash),
        }
    }
}
