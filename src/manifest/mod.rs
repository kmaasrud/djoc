use crate::{document::DocumentType, Author};
use serde::Deserialize;
use std::path::PathBuf;
use toml::value::Datetime;

mod serde_impls;

#[derive(Deserialize)]
#[serde(default)]
pub struct Common {
    pub locale: String,
    pub number_sections: bool,
}

impl Default for Common {
    fn default() -> Self {
        Self {
            locale: "en_US".into(),
            number_sections: false,
        }
    }
}

#[derive(Deserialize)]
pub struct GlobalManifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
    #[allow(dead_code)]
    #[serde(flatten)]
    common: Common,
}

#[derive(Deserialize)]
pub struct DocumentManifest {
    pub title: String,
    pub date: Option<Datetime>,
    #[serde(default, alias = "author")]
    pub authors: Vec<Author>,
    #[serde(default, alias = "chapter")]
    pub chapters: Vec<ChapterManifest>,
    #[serde(default, alias = "type")]
    pub document_type: DocumentType,
    #[serde(flatten)]
    pub common: Common,
}

pub struct ChapterManifest {
    pub title: String,
    pub path: PathBuf,
}
