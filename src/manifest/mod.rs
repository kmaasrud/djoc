use serde::Deserialize;
use std::path::PathBuf;

mod serde_impls;

#[derive(Default, Deserialize)]
pub struct DocumentManifest {
    pub title: String,
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
