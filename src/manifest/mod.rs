use serde::Deserialize;
use toml::value::Datetime;
use std::path::PathBuf;

mod serde_impls;

#[derive(Deserialize)]
pub struct DocumentManifest {
    pub title: String,
    pub date: Option<Datetime>,
    pub authors: Vec<AuthorManifest>,
    pub chapters: Vec<ChapterManifest>,
}

pub struct AuthorManifest {
    pub name: String,
    pub organization: Option<String>,
}

pub struct ChapterManifest {
    pub title: String,
    pub path: PathBuf,
}
