use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::error::Error;

#[allow(dead_code)]
pub enum ConfigFile {
    Base,
    Token,
    Config,
}

pub fn get_config_path(file: ConfigFile) -> Result<PathBuf> {
    let home_dir = dirs::home_dir();
    let config_dir = home_dir
        .context(Error::Config)?
        .join(".config")
        .join("rspotify-cli");
    if !config_dir.exists() {
        fs::create_dir_all(config_dir.clone()).context(Error::Config)?;
    }
    match file {
        ConfigFile::Base => Ok(config_dir),
        ConfigFile::Token => Ok(config_dir.join("token")),
        ConfigFile::Config => Ok(config_dir.join("config")),
    }
}
