use super::BuilderManifest;
use crate::{document::DocumentType, Author};
use serde::Deserialize;
use std::path::PathBuf;
use toml::value::Datetime;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DocumentManifest {
    pub title: String,
    pub date: Option<Datetime>,
    #[serde(default, alias = "author")]
    pub authors: Vec<Author>,
    #[serde(default, alias = "text")]
    pub texts: Vec<PathBuf>,
    #[serde(default, alias = "type")]
    pub document_type: DocumentType,
    pub locale: Option<String>,
    #[serde(flatten)]
    pub(crate) builder: BuilderManifest,
}
