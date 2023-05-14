use std::{
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
};

use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// File types stored in the config directory
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ConfigFile {
    Token,
    Config,
}

/// Config values
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub volume_increment: u8,
}

// TODO: move to rspotify-cli instead
pub fn get_config_path(file_name: ConfigFile) -> Result<PathBuf> {
    let config_dir = home_dir()
        .context(Error::Config)?
        .join(".config")
        .join("rspotify-cli");

    if !config_dir.exists() {
        create_dir_all(config_dir.clone())?;
    }

    let config_file = PathBuf::new().join(config_dir).join(match file_name {
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

#[allow(clippy::module_name_repetitions)]
pub fn load_config() -> Result<Config> {
    let config_file = get_config_path(ConfigFile::Config)?;

    let config = config::Config::builder()
        .set_default("redirect_uri", "http://localhost:8000/callback")?
        .set_default("volume_increment", 10u8)?
        .add_source(config::File::from(config_file.clone()))
        .add_source(config::Environment::with_prefix("SPOTIFY"))
        .build()?
        .try_deserialize::<Config>()
        .context(Error::IncompleteConfig(config_file.display().to_string()))?;

    Ok(config)
}
