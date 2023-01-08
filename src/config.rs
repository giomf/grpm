use std::{path::Path, fs};
use serde::Deserialize;
use toml;


#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub install_path: String
}

pub fn parse_config(path: &Path) -> Config{
    let content = fs::read_to_string(path).unwrap();
    let config: Config = toml::from_str(&content).unwrap();
    
    Config {
        install_path: shellexpand::tilde(&config.install_path).to_string(),
        ..config
    }
}