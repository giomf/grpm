use serde::{Deserialize, Deserializer};
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub token: Option<String>,
    #[serde(deserialize_with = "string_to_path")]
    pub install_path: PathBuf,
}

fn string_to_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let path = PathBuf::from(shellexpand::tilde(&buf).as_ref());
    Ok(path)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            install_path: get_default_config_path().join("config.toml"),
            token: None,
        }
    }
}

pub fn get_config() -> Config {
    let config_path = get_default_config_path().join("config.toml");
    if config_path.exists() {
        let content = fs::read_to_string(config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();
        return config;
    }
    Config::default()
}

pub fn create_default_folders() {
    fs::create_dir_all(get_default_config_path()).unwrap();
    fs::create_dir_all(get_default_data_path()).unwrap();
}

fn get_default_config_path() -> PathBuf {
    let base_path = std::env::var("XDG_CONFIG_HOME").unwrap_or("~/.config".to_string());
    let base_path = shellexpand::tilde(&base_path);
    Path::new(base_path.as_ref()).join("grpm")
}

pub fn get_default_data_path() -> PathBuf {
    let base_path = std::env::var("XDG_DATA_HOME").unwrap_or("~/.local/share".to_string());
    let base_path = shellexpand::tilde(&base_path);
    Path::new(base_path.as_ref()).join("grpm")
}
