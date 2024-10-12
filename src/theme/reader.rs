use crate::theme::{Theme, TomlTheme};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn parse_theme(custom_config_path: Option<String>) -> Result<Theme, ThemeError> {
    let toml_theme = match custom_config_path {
        Some(path) => parse_from_path(PathBuf::from(path))?,
        None => {
            let config_dir = get_config_dir()?;
            let path = config_dir.join("tailspin").join("theme.toml");
            parse_from_default_path(path)?
        }
    };

    Ok(Theme::from(toml_theme))
}

fn parse_from_path(path: PathBuf) -> Result<TomlTheme, ThemeError> {
    read_and_parse_toml(&path)
}

fn parse_from_default_path(path: PathBuf) -> Result<TomlTheme, ThemeError> {
    match read_and_parse_toml(&path) {
        Ok(toml_theme) => Ok(toml_theme),
        Err(ThemeError::FileNotFound) => Ok(TomlTheme::default()),
        Err(e) => Err(e),
    }
}

fn read_and_parse_toml(path: &Path) -> Result<TomlTheme, ThemeError> {
    if path.exists() {
        let file_content = fs::read_to_string(path).map_err(ThemeError::Read)?;
        toml::from_str::<TomlTheme>(&file_content).map_err(ThemeError::Parsing)
    } else {
        Err(ThemeError::FileNotFound)
    }
}

fn get_config_dir() -> Result<PathBuf, ThemeError> {
    if let Ok(xdg_config_dir) = env::var("XDG_CONFIG_HOME") {
        let home_dir = shellexpand::tilde(&xdg_config_dir).into_owned();
        Ok(PathBuf::from(home_dir))
    } else {
        let home_dir = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map_err(ThemeError::HomeEnvironment)?;

        Ok(PathBuf::from(home_dir))
    }
}

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("could not read the toml file: {0}")]
    Read(#[source] io::Error),
    #[error("could not parse the toml file: {0}")]
    Parsing(#[source] toml::de::Error),
    #[error("could not find the toml file")]
    FileNotFound,
    #[error("could not determine the default home environment: {0}")]
    HomeEnvironment(#[source] env::VarError),
}
