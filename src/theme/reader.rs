use crate::theme::{Theme, TomlTheme};
use std::path::PathBuf;
use std::{env, fs};
use thiserror::Error;

pub fn parse_theme(custom_config_path: Option<String>) -> Result<Theme, ThemeError> {
    let toml_theme = match custom_config_path {
        Some(path) => parse_from_custom_path(path),
        None => parse_from_default_home_dirs(),
    }?;

    Ok(Theme::from(toml_theme))
}

pub fn parse_from_custom_path(path: String) -> Result<TomlTheme, ThemeError> {
    let full_path_to_toml = PathBuf::from(path);

    match full_path_to_toml.exists() {
        true => {
            let file_content = fs::read_to_string(full_path_to_toml).or(Err(ThemeError::Read))?;
            Ok(toml::from_str::<TomlTheme>(&file_content).map_err(|_| ThemeError::Parsing)?)
        }
        false => Err(ThemeError::FileNotFound),
    }
}

pub fn parse_from_default_home_dirs() -> Result<TomlTheme, ThemeError> {
    let config_dir = get_config_dir()?;
    let full_path_to_toml = config_dir.join("tailspin").join("theme.toml");

    match full_path_to_toml.exists() {
        true => {
            let file_content = fs::read_to_string(full_path_to_toml).or(Err(ThemeError::Read))?;
            Ok(toml::from_str::<TomlTheme>(&file_content).map_err(|_| ThemeError::Parsing)?)
        }
        false => Ok(TomlTheme::default()),
    }
}

fn get_config_dir() -> Result<PathBuf, ThemeError> {
    match env::var("XDG_CONFIG_HOME").ok() {
        Some(xdg_config_dir) => {
            let home_dir = shellexpand::tilde(&xdg_config_dir).into_owned();
            Ok(PathBuf::from(home_dir))
        }
        None => {
            let home_dir = env::var("HOME")
                .or_else(|_| env::var("USERPROFILE"))
                .map_err(|_| ThemeError::HomeEnvironment)?;

            Ok(PathBuf::from(home_dir))
        }
    }
}

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("could not read the toml file")]
    Read,
    #[error("could not parse the toml file")]
    Parsing,
    #[error("could not find the toml file")]
    FileNotFound,
    #[error("could not determine the default home environment")]
    HomeEnvironment,
}
