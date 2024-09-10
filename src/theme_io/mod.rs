use crate::theme::raw::Theme;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

pub fn load_theme(path: Option<&str>) -> Theme {
    let config_dir = if let Ok(xdg_config_dir) = env::var("XDG_CONFIG_HOME") {
        let expanded_path = shellexpand::tilde(&xdg_config_dir).into_owned();
        PathBuf::from(expanded_path)
    } else {
        let home_dir = env::var("HOME")
            .or(env::var("USERPROFILE"))
            .expect("HOME directory not set");
        PathBuf::from(home_dir).join(".config")
    };

    let default_config_path = config_dir.join("tailspin").join("config.toml");

    let path = path.or_else(|| {
        if default_config_path.exists() {
            Some(default_config_path.to_str().expect("Invalid path"))
        } else {
            None
        }
    });

    match path {
        Some(path) => {
            let contents = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("Could not read file '{path}':\n\n{err}");
                    exit(1);
                }
            };

            match toml::from_str::<Theme>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("Could not deserialize file '{path}':\n\n{err}");
                    exit(1);
                }
            }
        }

        None => Theme::default(),
    }
}
