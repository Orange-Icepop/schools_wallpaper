use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub wallpaper_dir: String,
    pub use_override_wallpaper: bool,
    pub override_wallpaper: String,
}

impl Config {
    pub fn new(
        wallpaper_dir: &str,
        use_override_wallpaper: bool,
        override_wallpaper: &str,
    ) -> Self {
        Self {
            wallpaper_dir: wallpaper_dir.into(),
            use_override_wallpaper,
            override_wallpaper: override_wallpaper.into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("", false, "")
    }
}

fn write_config(config: &Config) -> Result<()> {
    let config_str = toml::to_string_pretty(config)
        .with_context(|| "Failed to serialize config")?;
    fs::write(env::current_dir()?.join("config.toml"), config_str)
        .with_context(|| "Failed to write config file")?;
    Ok(())
}

pub fn read_all_config() -> Result<Config> {
    let config_path = env::current_dir()?.join("config.toml");
    if !config_path.exists() {
        let mut default_config = Config::default();
        write_config(&default_config)?;
        create_dir_structure(&env::current_dir()?)?;
        default_config.wallpaper_dir = env::current_dir()?.to_string_lossy().to_string();
        Ok(default_config)
    } else {
        let config_str =
            fs::read_to_string(config_path).with_context(|| "Failed to read config file")?;
        let mut config: Config = toml::from_str(&config_str)
            .with_context(|| "Failed to parse config file")?;
        if config.wallpaper_dir.is_empty() {
            config.wallpaper_dir = env::current_dir()?.to_string_lossy().to_string();
        }
        create_dir_structure(&PathBuf::from(&config.wallpaper_dir))?;
        Ok(config)
    }
}

fn create_dir_structure(path: impl AsRef<Path>) -> Result<()> {
    let conf = path.as_ref().join("imgs");
    create_dir(&conf.join("1"))?;
    create_dir(&conf.join("2"))?;
    create_dir(&conf.join("3"))?;
    create_dir(&conf.join("4"))?;
    create_dir(&conf.join("5"))?;
    create_dir(&conf.join("6"))?;
    create_dir(&conf.join("7"))?;
    Ok(())
}

fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    if !Path::new(path.as_ref()).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
