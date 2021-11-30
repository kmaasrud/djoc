use crate::utils::read_file;

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub title: String,
    pub authors: Vec<String>,
    pub date: Option<String>,

    pub build: BuildConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_content = read_file(path)?;
        Ok(toml::from_str(&config_content)?)
    }
}

#[derive(Debug, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    pub filename: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            filename: "mdoc".to_owned(),
        }
    }
}
