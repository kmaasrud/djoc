use crate::utils::{kebab, read_file};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};

pub const READABLE: &[FormatItem] =
    format_description!("[day padding:none] [month repr:long case_sensitive:false] [year]");

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub(crate) src: Option<toml::Value>,

    // Metadata
    pub title: String,
    pub authors: Vec<String>,
    pub date: Option<String>,

    // Subtables
    pub bib: BibConfig,
    pub build: BuildConfig,
    pub latex: LatexConfig,
    pub style: StyleConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_content = read_file(path)?;
        Ok(toml::from_str(&config_content)?)
    }

    pub(crate) fn latex_header(&self) -> Option<String> {
        if self.latex.head.is_empty() && self.latex.packages.is_empty() {
            None
        } else {
            let mut header = String::new();
            header.push_str(&self.latex.head);
            let packages: String = self
                .latex
                .packages
                .iter()
                .map(|package| format!("\\usepackage{{{}}}\n", package))
                .collect();
            header.push_str(&packages);
            Some(header)
        }
    }

    pub(crate) fn date(&self) -> String {
        match self.date.as_deref() {
            Some("now") => {
                let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
                // FIXME: Dangerous unwrap
                now.format(&READABLE).unwrap()
            }
            Some(date) => date.to_owned(),
            None => String::default(),
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
            bib: BibConfig::default(),
            build: BuildConfig::default(),
            latex: LatexConfig::default(),
            style: StyleConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BibConfig {
    pub csl: String,
    pub src: Option<String>,
}

impl Default for BibConfig {
    fn default() -> Self {
        Self {
            csl: "apa".to_owned(),
            src: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    pub filename: Option<String>,
    pub tidy_logs: bool,
    pub output: String,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            filename: None,
            tidy_logs: true,
            output: "pdf".to_owned(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct StyleConfig {
    pub number_sections: bool,
    pub document_class: Option<String>,
    pub class_options: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct LatexConfig {
    pub head: String,
    pub packages: Vec<String>,
    pub title_script: Option<String>,
}
