use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Datetime;

use super::BuilderManifest;
use crate::{document::DocumentType, Author};

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
    #[serde(flatten)]
    pub(crate) builder: BuilderManifest,
}
