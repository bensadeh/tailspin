use crate::theme::{Theme, TomlTheme};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn parse_theme(custom_config_path: Option<String>) -> Result<Theme, ThemeError> {
    let toml_theme = if let Some(path) = custom_config_path {
        read_and_parse_toml(&PathBuf::from(path))?
    } else {
        let default_path = get_config_dir()?.join("tailspin").join("theme.toml");
        match read_and_parse_toml(&default_path) {
            Ok(theme) => theme,
            Err(ThemeError::FileNotFound) => TomlTheme::default(),
            Err(e) => return Err(e),
        }
    };

    Ok(Theme::from(toml_theme))
}

fn read_and_parse_toml(path: &Path) -> Result<TomlTheme, ThemeError> {
    let content = fs::read_to_string(path).map_err(|err| match err.kind() {
        io::ErrorKind::NotFound => ThemeError::FileNotFound,
        _ => ThemeError::Read(err),
    })?;

    toml::from_str::<TomlTheme>(&content).map_err(ThemeError::Parsing)
}

fn get_config_dir() -> Result<PathBuf, ThemeError> {
    ["XDG_CONFIG_HOME", "HOME", "USERPROFILE"]
        .iter()
        .find_map(|&var| env::var(var).ok())
        .map(|dir| PathBuf::from(shellexpand::tilde(&dir).into_owned()))
        .ok_or_else(|| ThemeError::HomeEnvironment(env::VarError::NotPresent))
}

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("could not read the TOML file: {0}")]
    Read(#[source] io::Error),
    #[error("could not parse the TOML file: {0}")]
    Parsing(#[source] toml::de::Error),
    #[error("could not find the TOML file")]
    FileNotFound,
    #[error("could not determine the home environment: {0}")]
    HomeEnvironment(#[source] env::VarError),
}
