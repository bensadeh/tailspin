use crate::config::Config;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

const DEFAULT_CONFIG: &str = include_str!("../../data/config.toml");

pub fn load_config(path: Option<String>) -> Config {
    let config_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_dir) => {
            let expanded_path = shellexpand::tilde(&xdg_config_dir).into_owned();
            PathBuf::from(expanded_path)
        }
        Err(_) => {
            let home_dir = env::var("HOME").expect("HOME directory not set");
            PathBuf::from(home_dir).join(".config")
        }
    };

    let default_config_path = config_dir.join("tailspin").join("config.toml");

    let path = path.or_else(|| {
        if default_config_path.exists() {
            Some(
                default_config_path
                    .to_str()
                    .expect("Invalid path")
                    .to_owned(),
            )
        } else {
            None
        }
    });

    match path {
        Some(path) => {
            let p = &PathBuf::from(path);
            let contents = fs::read_to_string(p).expect("Could not read file");

            match toml::from_str::<Config>(&contents) {
                Ok(config) => config,
                Err(err) => {
                    println!("Could not deserialize file:\n\n{}", err);
                    exit(1);
                }
            }
        }
        None => match toml::from_str(DEFAULT_CONFIG) {
            Ok(config) => config,
            Err(err) => {
                println!("Could not deserialize default config:\n\n{}", err);
                exit(1);
            }
        },
    }
}

pub fn create_default_config() {
    let target_config_path = match env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_dir) => {
            let expanded_path = shellexpand::tilde(&xdg_config_dir).into_owned();
            PathBuf::from(expanded_path)
        }
        Err(_) => {
            let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
            PathBuf::from(home_dir).join(".config")
        }
    }
    .join("tailspin")
    .join("config.toml");

    let tilde_path = target_config_path.to_str().expect("Invalid path").replace(
        env::var("HOME").expect("HOME directory not set").as_str(),
        "~",
    );

    match target_config_path.try_exists() {
        Ok(true) => {
            eprintln!("Config file already exists at {}", tilde_path);
            exit(1);
        }
        Err(err) => {
            eprintln!("Failed to check if file {} exists: {}", tilde_path, err);
            exit(1);
        }
        _ => {}
    }

    if let Some(parent_path) = target_config_path.parent() {
        match fs::create_dir_all(parent_path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to create the directory for {}: {}", tilde_path, err);
                exit(1);
            }
        }
    }

    match File::create(&target_config_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(DEFAULT_CONFIG.as_bytes()) {
                eprintln!(
                    "Failed to write to the config file at {}: {}",
                    tilde_path, err
                );
                exit(1);
            }

            println!("Config file generated successfully at {}", tilde_path);
        }
        Err(err) => {
            eprintln!(
                "Failed to create the config file at {}: {}",
                tilde_path, err
            );
            exit(1);
        }
    }
}

pub fn default_config() -> &'static str {
    DEFAULT_CONFIG
}
