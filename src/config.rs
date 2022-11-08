use std::{
    fs::{create_dir_all, read_to_string, OpenOptions},
    path::PathBuf,
};

use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigFile {
    Token,
    Config,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

pub fn get_config_path(file_name: ConfigFile) -> Result<PathBuf> {
    let config_dir = home_dir()
        .context(Error::Config)?
        .join(".config")
        .join("rspotify-cli");

    if !config_dir.exists() {
        create_dir_all(config_dir.clone())?;
    }

    let mut config_file = PathBuf::new();
    config_file.push(config_dir);
    config_file.push(match file_name {
        ConfigFile::Token => "token.json",
        ConfigFile::Config => "config.toml",
    });

    if !config_file.exists() {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(config_file.clone())?;
    }

    Ok(config_file)
}

pub fn load_config() -> Result<Config> {
    let config_file = get_config_path(ConfigFile::Config)?;
    let contents = read_to_string(config_file.clone())?;
    let config = toml::from_str::<Config>(&contents)
        .context(Error::IncompleteConfig(config_file.display().to_string()))?;

    Ok(config)
}
