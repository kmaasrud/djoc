use crate::{config::Config, utils::find_root, Chapter, Document};

use anyhow::{anyhow, bail, Context, Result};
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

            SourceType::Dir(ref path) => {
                let config = self
                    .config
                    .unwrap_or(Config::from_file(path.join("mdoc.toml"))?);
                let chapters = load_chapters(path, &config)?;

                (config, chapters)
            }

            SourceType::None => {
                let root = find_root()?;
                let config = self
                    .config
                    .unwrap_or(Config::from_file(root.join("mdoc.toml"))?);
                let chapters = load_chapters(root, &config)?;

                (config, chapters)
            }
        };

        if chapters.is_empty() {
            bail!("No chapters found.");
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

fn load_chapters<P: AsRef<Path>>(path: P, config: &Config) -> Result<Vec<Chapter>> {
    let walk = |path: &Path| {
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
    };

    match &config.src {
        Some(toml::Value::Array(paths)) => Ok(paths
            .iter()
            .filter_map(|val| val.as_str())
            .map(|p| Chapter::load(Path::new(p)))
            .filter_map(|ch| ch.map_err(|err| warn!("{}. Skipping...", err)).ok())
            .collect()),

        Some(toml::Value::String(path)) => {
            let path = Path::new(&path);
            if !path.exists() {
                bail!("You specified a path {:?}, which does not exist.", path)
            } else if !path.is_dir() {
                Err(anyhow!(
                    "The src field must be a directory or a list of files."
                ))
                .with_context(|| format!("{:?} is not a directory.", path))
            } else {
                Ok(walk(path))
            }
        }

        _ => Ok(walk(path.as_ref())),
    }
}
