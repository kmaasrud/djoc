use crate::{document::DocumentType, Author};
use serde::Deserialize;
use std::path::PathBuf;
use toml::value::Datetime;

mod serde_impls;

#[derive(Deserialize)]
pub struct Common {
    pub output_format: Option<String>,
    pub locale: Option<String>,
    pub number_sections: Option<bool>,
}

impl Common {
    fn merge(self, other: Self) -> Self {
        Self {
            output_format: self.output_format.or(other.output_format),
            locale: self.locale.or(other.locale),
            number_sections: self.number_sections.or(other.number_sections),
        }
    }
}

#[derive(Deserialize)]
pub struct GlobalManifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
    #[serde(flatten)]
    pub common: Common,
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
    pub title: Option<String>,
    pub path: PathBuf,
}
