use crate::utils::read_file;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn latex_authors(&self) -> String {
        self.authors.join(" \\and ")
    }

    pub fn date(&self) -> String {
        self.date
            .to_owned()
            .unwrap_or_default()
            .replace("today", "\\today")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "Document title".to_owned(),
            authors: vec![],
            date: None,
            build: BuildConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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
