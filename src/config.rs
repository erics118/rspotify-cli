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

#[derive(Debug, Deserialize, Serialize, Default)]
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

    let mut file = PathBuf::new();
    file.push(config_dir);
    file.push(match file_name {
        ConfigFile::Token => "token",
        ConfigFile::Config => "config.toml",
    });

    if !file.exists() {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file.clone())?;
    }
    Ok(file)
}

pub fn load_config() -> Result<Config> {
    let contents = read_to_string(get_config_path(ConfigFile::Config)?)?;
    toml::from_str::<Config>(&contents).context(Error::IncompleteConfig)
}
