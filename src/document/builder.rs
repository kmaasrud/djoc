use crate::{Document, Chapter, config::Config};

use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

enum SourceType {
    File(PathBuf),
    Dir(PathBuf),
    None,
}

#[allow(dead_code)]
pub struct DocumentBuilder {
    source: SourceType,
    config: Option<Config>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        DocumentBuilder {
            source: SourceType::None,
            config: None,
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

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<Document> {
        let (_, chapters) = match self.source {
            SourceType::File(ref path) => (PathBuf::from(""), vec![Chapter::load(path)?]),
            SourceType::Dir(ref path) => {
                let mut chapters = Vec::new();
                for entry in WalkBuilder::new(path).build() {
                    if let Ok(entry) = entry {
                        if entry.path().is_file() {
                            chapters.push(Chapter::load(entry.path())?);
                        }
                    }
                }
                (path.join("mdoc.toml"), chapters)
            },
            SourceType::None => (PathBuf::from(""), vec![]),
        };
        
        let config = self.config.unwrap_or_else(|| Config::default());

        Ok(Document {
            chapters,
            config
        })
    }
}
