use serde::Deserialize;
use std::path::PathBuf;
use toml::value::Datetime;

mod serde_impls;

#[derive(Deserialize)]
#[serde(default)]
pub struct Common {
    format: String,
}

impl Default for Common {
    fn default() -> Self {
        Self {
            format: "pdf".into(),
        }
    }
}

#[derive(Deserialize)]
pub struct GlobalManifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
    #[serde(flatten)]
    common: Common,
}

#[derive(Deserialize)]
pub struct DocumentManifest {
    pub title: String,
    pub date: Option<Datetime>,
    #[serde(default, alias = "author")]
    pub authors: Vec<AuthorManifest>,
    #[serde(default, alias = "chapter")]
    pub chapters: Vec<ChapterManifest>,
    #[serde(default)]
    pub number_sections: bool,
    #[serde(flatten)]
    common: Common,
}

pub struct AuthorManifest {
    pub name: String,
    pub email: Option<String>,
    pub organization: Option<String>,
}

pub struct ChapterManifest {
    pub title: String,
    pub path: PathBuf,
}
