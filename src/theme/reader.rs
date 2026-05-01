use crate::theme::{Theme, TomlTheme};
use std::env;
use std::env::VarError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn parse_theme(custom_config_path: Option<&PathBuf>) -> Result<Theme, ThemeError> {
    if let Some(path) = custom_config_path {
        let toml_theme = read_and_parse_toml(path)?;
        return Ok(Theme::from(toml_theme));
    }

    let default_path = get_config_dir()?.join("tailspin").join("theme.toml");

    let toml_theme = match read_and_parse_toml(&default_path) {
        Ok(theme) => theme,
        Err(ThemeError::FileNotFound) => TomlTheme::default(),
        Err(e) => return Err(e),
    };

    Ok(Theme::from(toml_theme))
}

fn get_config_dir() -> Result<PathBuf, ThemeError> {
    expand_var_os("XDG_CONFIG_HOME")
        .or_else(|| expand_var_os("HOME").map(|home| home.join(".config")))
        .or_else(|| expand_var_os("USERPROFILE"))
        .ok_or(ThemeError::HomeEnvironment(VarError::NotPresent))
}

fn expand_var_os(key: &str) -> Option<PathBuf> {
    env::var_os(key)
        .and_then(|os_str| os_str.into_string().ok())
        .map(|s| shellexpand::tilde(&s).into_owned().into())
}
fn read_and_parse_toml(path: &Path) -> Result<TomlTheme, ThemeError> {
    let display_path = || path.display().to_string();

    let content = fs::read_to_string(path).map_err(|err| match err.kind() {
        io::ErrorKind::NotFound => ThemeError::FileNotFound,
        _ => ThemeError::Read(display_path(), err),
    })?;

    toml::from_str::<TomlTheme>(&content).map_err(|err| ThemeError::Parsing(display_path(), err))
}

#[derive(Debug, Error)]
pub enum ThemeError {
    #[error("could not read {0}: {1}")]
    Read(String, #[source] io::Error),

    #[error("could not parse {0}: {1}")]
    Parsing(String, toml::de::Error),

    #[error("could not find the TOML file")]
    FileNotFound,

    #[error("could not determine the home environment: {0}")]
    HomeEnvironment(#[source] VarError),
}
