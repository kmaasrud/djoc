use crate::{Document, Chapter};

use anyhow::Result;
use std::path::{Path, PathBuf};

enum SourceType {
    File(PathBuf),
    Dir(PathBuf),
    None,
}

#[allow(dead_code)]
pub struct DocumentBuilder {
    source: SourceType,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        DocumentBuilder {
            source: SourceType::None,
        }
    }

    pub fn source(mut self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        if path.is_dir() {
            self.source = SourceType::Dir(path.to_owned());
        } else if path.is_file() {
            self.source = SourceType::File(path.to_owned());
        }

        self
    }

    pub fn build(&self) -> Result<Document> {
        let (_, chapters) = match self.source {
            SourceType::File(ref path) => (PathBuf::from(""), vec![Chapter::load(path)?]),
            SourceType::Dir(ref path) => (path.join("mdoc.toml"), vec![]),
            SourceType::None => (PathBuf::from(""), vec![]),
        };

        Ok(Document {
            chapters,
        })
    }
}
