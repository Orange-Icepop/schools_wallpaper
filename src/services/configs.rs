use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub wallpaper_dir: String,
    pub override_wallpaper: String,
}

impl Config {
    pub fn new(wallpaper_dir: String, override_wallpaper: String) -> Self {
        Self {
            wallpaper_dir,
            override_wallpaper,
        }
    }
}

fn default_config() -> Config {
    Config {
        wallpaper_dir: String::from(""),
        override_wallpaper: String::from(""),
    }
}

fn write_config(config: &Config, path: &str) -> Result<()> {
    let config_str = toml::to_string_pretty(config).unwrap();
    fs::write(path, config_str)
}