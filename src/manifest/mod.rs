use crate::{document::DocumentType, Author};
use serde::Deserialize;
use std::path::PathBuf;
use toml::value::Datetime;

mod serde_impls;

#[derive(Clone, Deserialize)]
pub struct Output {
    pub name: Option<String>,
    pub format: OutputFormat,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    #[default]
    Pdf,
    Html,
    #[serde(alias = "tex")]
    Latex,
}

impl AsRef<str> for OutputFormat {
    fn as_ref(&self) -> &str {
        match self {
            OutputFormat::Pdf => "pdf",
            OutputFormat::Html => "html",
            OutputFormat::Latex => "tex",
        }
    }
}

#[derive(Deserialize)]
pub struct GlobalManifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DocumentManifest {
    pub title: String,
    pub date: Option<Datetime>,
    #[serde(default, alias = "author")]
    pub authors: Vec<Author>,
    #[serde(default, alias = "chapter")]
    pub chapters: Vec<ChapterManifest>,
    #[serde(default, alias = "type")]
    pub document_type: DocumentType,
    #[serde(alias = "output")]
    pub outputs: Vec<Output>,
    pub locale: Option<String>,
    pub number_sections: Option<bool>,
}

pub struct ChapterManifest {
    pub title: Option<String>,
    pub path: PathBuf,
}
