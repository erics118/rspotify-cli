//! Configuration for the CLI.

use std::{
    fs::{create_dir_all, OpenOptions},
    path::PathBuf,
};

use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// File types stored in the config directory.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ConfigFile {
    /// Token file.
    Token,

    /// Config file.
    Config,
}

/// Config values.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Client id for the Spotify API.
    pub client_id: String,

    /// Client secret for the Spotify API.
    pub client_secret: String,

    /// Redirect URI for the Spotify API.
    pub redirect_uri: String,

    /// Volume increment for the volume increment and decrement commands.
    pub volume_increment: u8,
}

/// Get a config file path from the config directory.
pub fn get_config_path(file_name: ConfigFile) -> Result<PathBuf> {
    let config_dir = match std::env::var("XDG_CONFIG_HOME") {
        Ok(path) => PathBuf::from(path),
        Err(_) => home_dir().context(Error::Config)?.join(".config"),
    }
    .join("rspotify-cli");

    if !config_dir.exists() {
        create_dir_all(config_dir.clone())?;
    }

    let config_file = config_dir.join(match file_name {
        ConfigFile::Token => "token.json",
        ConfigFile::Config => "config.toml",
    });

    if !config_file.exists() {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(config_file.clone())
            .context(Error::Config)?;
    }

    Ok(config_file)
}

/// Load config from the config file
pub fn load_config() -> Result<Config> {
    let config_file = get_config_path(ConfigFile::Config)?;

    let config = config::Config::builder()
        .set_default("redirect_uri", "http://localhost:8000/callback")?
        .set_default("volume_increment", 10)?
        .add_source(config::File::from(config_file.clone()))
        .add_source(config::Environment::with_prefix("SPOTIFY"))
        .build()?
        .try_deserialize::<Config>()
        .context(Error::IncompleteConfig(config_file.display().to_string()))?;

    Ok(config)
}
