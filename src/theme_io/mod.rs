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
            let contents = fs::read_to_string(path).expect("Could not read file");

            match toml::from_str::<Theme>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    println!("Could not deserialize file '{path}':\n\n{err}");
                    exit(1);
                }
            }
        }
        None => match toml::from_str("") {
            Ok(config) => config,
            Err(err) => {
                println!("Could instantiate empty config using default values:\n\n{err}");
                exit(1);
            }
        },
    }
}
