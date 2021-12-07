use crate::utils::{kebab, read_file};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub(crate) src: Option<toml::Value>,

    // Metadata
    pub title: String,
    pub authors: Vec<String>,
    pub date: Option<String>,

    // LaTeX configuration
    pub latex_head: String,

    // Subtables
    pub bib: BibConfig,
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
        self.build
            .filename
            .to_owned()
            .unwrap_or_else(|| kebab(&self.title))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            src: None,
            title: "Document title".to_owned(),
            authors: vec![],
            date: None,
            latex_head: "".to_owned(),
            bib: BibConfig::default(),
            build: BuildConfig::default(),
            style: StyleConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BibConfig {
    pub csl: String,
}

impl Default for BibConfig {
    fn default() -> Self {
        Self { csl: "apa".to_owned() }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    pub filename: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct StyleConfig {
    pub number_sections: bool,
}
