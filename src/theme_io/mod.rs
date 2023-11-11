use crate::theme::Theme;

use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

pub fn load_theme(path: Option<String>) -> Theme {
    let config_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_dir) => {
            let expanded_path = shellexpand::tilde(&xdg_config_dir).into_owned();
            PathBuf::from(expanded_path)
        }
        Err(_) => {
            let home_dir = env::var("HOME")
                .or(env::var("USERPROFILE"))
                .expect("HOME directory not set");
            PathBuf::from(home_dir).join(".config")
        }
    };

    let default_config_path = config_dir.join("tailspin").join("config.toml");

    let path = path.or_else(|| {
        if default_config_path.exists() {
            Some(default_config_path.to_str().expect("Invalid path").to_owned())
        } else {
            None
        }
    });

    match path {
        Some(path) => {
            let p = &PathBuf::from(path);
            let contents = fs::read_to_string(p).expect("Could not read file");

            match toml::from_str::<Theme>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    println!("Could not deserialize file:\n\n{}", err);
                    exit(1);
                }
            }
        }
        None => match toml::from_str("") {
            Ok(config) => config,
            Err(err) => {
                println!("Could instantiate empty config using default values:\n\n{}", err);
                exit(1);
            }
        },
    }
}
