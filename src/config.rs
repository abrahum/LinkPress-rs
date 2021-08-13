use chrono::prelude::Local;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net;
use std::path::PathBuf;
use std::str::FromStr;
use toml;

pub const CONFIG_PATH: &str = "LinkPress.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub site: SiteConfig,
    pub serve: ServeConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SiteConfig {
    site_name: String,
    subtitle: String,
    author: String,
    since: i32,
    pub theme: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServeConfig {
    pub host: net::IpAddr,
    pub port: u16,
}

pub fn new() -> Config {
    let now = Local::now();
    let new_config = Config {
        site: SiteConfig {
            site_name: String::from("LinkPress"),
            subtitle: String::from("Just a linkpress-rs website"),
            author: String::from("Unkown"),
            since: now.year(),
            theme: String::from("Default"),
        },
        serve: ServeConfig {
            host: net::IpAddr::from_str("127.0.0.1").unwrap(),
            port: 4040,
        },
    };
    new_config
}

pub fn load_config() -> Config {
    let config: Config;
    if !PathBuf::from(CONFIG_PATH).exists() {
        config = new().save(None);
    } else {
        let config_toml = fs::read_to_string(PathBuf::from(CONFIG_PATH)).unwrap();
        config = toml::from_str(&config_toml).unwrap();
    };
    config
}

impl Config {
    pub fn save(self, dir: Option<&str>) -> Config {
        let config_toml = toml::to_string(&self).unwrap();
        let dir_name: PathBuf;
        match dir {
            Some(dn) => dir_name = PathBuf::from(dn).join(CONFIG_PATH),
            None => dir_name = PathBuf::from(CONFIG_PATH),
        }
        fs::write(PathBuf::from(dir_name), config_toml).expect("LinkPress.toml保存错误");
        self
    }
}
