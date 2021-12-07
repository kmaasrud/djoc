use crate::utils::{read_file, kebab};

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
    pub style: StyleConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_content = read_file(path)?;
        Ok(toml::from_str(&config_content)?)
    }

    pub(crate) fn latex_authors(&self) -> String {
        self.authors.join(" \\and ")
    }

    pub(crate) fn date(&self) -> String {
        self.date
            .to_owned()
            .unwrap_or_default()
            .replace("today", "\\today")
    }

    pub(crate) fn number_sections(&self) -> String {
        if self.style.number_sections {
            "\\setcounter{secnumdepth}{5}".to_owned()
        } else {
            "\\setcounter{secnumdepth}{-\\maxdimen}".to_owned()
        }
    }

    pub fn filename(&self) -> String {
        self.build.filename.to_owned().unwrap_or_else(|| kebab(&self.title))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "Document title".to_owned(),
            authors: vec![],
            date: None,
            build: BuildConfig::default(),
            style: StyleConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    pub filename: Option<String>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            filename: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct StyleConfig {
    pub number_sections: bool,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            number_sections: false,
        }
    }
}
