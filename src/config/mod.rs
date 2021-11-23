use crate::utils::read_file;

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    title: String,
    author: String,
    date: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_content = read_file(path)?;
        Ok(toml::from_str(&config_content)?)
    }
}
