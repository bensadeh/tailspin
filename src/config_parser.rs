// use std::fs;
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
// use toml;
//
// const DEFAULT_CONFIG: &str = include_str!("../data/config.toml");
//
// #[derive(Debug, Deserialize)]
// pub struct Config {
//     // define the fields of your configuration here, as they appear in your TOML file
//     // For example:
//     // pub key: String,
//     // pub value: i32,
// }
//
// pub fn load_config(path: Option<String>) -> Config {
//     match path {
//         Some(path) => {
//             let p = &Path::new(&path);
//             let contents = fs::read_to_string(p).expect("Could not read file");
//
//             toml::from_str(&contents).expect("Could not deserialize file")
//         }
//         None => toml::from_str(DEFAULT_CONFIG).expect("Could not deserialize default config"),
//     }
// }
