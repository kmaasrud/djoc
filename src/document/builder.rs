use crate::{config::Config, utils::find_root, Chapter, Document};

use anyhow::Result;
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::{Path, PathBuf};

enum SourceType {
    File(PathBuf),
    Dir(PathBuf),
    None,
}

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
        let (config, chapters) = match self.source {
            SourceType::File(ref path) => {
                (self.config.unwrap_or_default(), vec![Chapter::load(path)?])
            }
            SourceType::Dir(ref path) => (
                self.config
                    .unwrap_or(Config::from_file(path.join("mdoc.toml"))?),
                load_chapters(path),
            ),
            SourceType::None => {
                let root = find_root()?;
                let config = self
                    .config
                    .unwrap_or(Config::from_file(root.join("mdoc.toml"))?);
                (config, load_chapters(root))
            }
        };

        if chapters.is_empty() {
            anyhow::bail!("No chapters found.");
        }

        debug!("Building document with {} chapters.", chapters.len());
        debug!("Using config: {:#?}", config);

        Ok(Document { chapters, config })
    }
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn load_chapters<P: AsRef<Path>>(path: P) -> Vec<Chapter> {
    let md_types = TypesBuilder::new()
        .add_defaults()
        .select("markdown")
        .build()
        .unwrap();

    WalkBuilder::new(path)
        .types(md_types)
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| Chapter::load(entry.path()))
        .filter_map(|ch| ch.map_err(|err| warn!("{}. Skipping...", err)).ok())
        .collect()
}
