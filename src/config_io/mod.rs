use crate::config::Config;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs};

const DEFAULT_CONFIG: &str = include_str!("../../data/config.toml");

pub fn load_config(path: Option<String>) -> Config {
    // Obtain the home directory
    let home_dir = env::var("HOME").expect("HOME directory not set");
    let home_path = PathBuf::from(home_dir);

    // Construct the path to the default configuration file
    let default_config_path = home_path.join(".config/tailspin/config.toml");

    let path = path.or_else(|| {
        // If no path is provided, and if a config exists at the default path, use it
        if default_config_path.exists() {
            Some(
                default_config_path
                    .to_str()
                    .expect("Invalid path")
                    .to_owned(),
            )
        } else {
            // If no path is provided and no config exists at the default path, use default
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
        // If no file was found, use the default configuration
        None => match toml::from_str(DEFAULT_CONFIG) {
            Ok(config) => config,
            Err(err) => {
                println!("Could not deserialize default config:\n\n{}", err);
                exit(1);
            }
        },
    }
}

pub fn generate_default_config() {
    const TARGET_CONFIG_PATH: &str = "~/.config/tailspin/config.toml";

    let home_dir = env::var("HOME").expect("Failed to get HOME environment variable");
    let expanded_path = shellexpand::tilde(TARGET_CONFIG_PATH).into_owned();
    let tilde_path = expanded_path.replace(&home_dir, "~");
    let path = Path::new(&expanded_path);

    match path.try_exists() {
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

    if let Some(parent_path) = path.parent() {
        match fs::create_dir_all(parent_path) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to create the directory for {}: {}", tilde_path, err);
                exit(1);
            }
        }
    }

    match File::create(path) {
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
