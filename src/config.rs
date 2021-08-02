use chrono::prelude::Local;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub site: SiteConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SiteConfig {
    site_name: String,
    since: i32,
    pub theme: String,
}

fn create_config(config_path: PathBuf) -> Config {
    let now = Local::now();
    let new_config = Config {
        site: SiteConfig {
            site_name: String::from("A linkpress-rs website"),
            since: now.year(),
            theme: String::from("Default"),
        },
    };
    let new_config_toml = toml::to_string(&new_config).unwrap();
    fs::write(config_path, new_config_toml).unwrap();
    new_config
}

pub fn load_config() -> Config {
    let config: Config;
    let config_file_path = PathBuf::from("linkpress.toml");
    if !config_file_path.exists() {
        config = create_config(config_file_path);
    } else {
        let config_toml = fs::read_to_string(config_file_path).unwrap();
        config = toml::from_str(&config_toml).unwrap();
    };
    config
}
