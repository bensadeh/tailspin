use crate::theme::*;

impl From<TomlTheme> for Theme {
    fn from(toml: TomlTheme) -> Self {
        Theme {
            uuid: toml.uuid.map_or_else(UuidConfig::default, UuidConfig::from),
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
