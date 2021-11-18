use crate::{config::Config, Chapter, Document};

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
        let (config, chapters) = match self.source {
            SourceType::File(ref path) => (Config::default(), vec![Chapter::load(path)?]),
            SourceType::Dir(ref path) => (Config::default(), load_chapters(path)),
            SourceType::None => {
                let root = {
                    let mut path: PathBuf = std::env::current_dir().unwrap();
                    let look_for = Path::new("mdoc.toml");
                    loop {
                        path.push(look_for);
                        if path.is_file() {
                            path.pop();
                            break path;
                        }
                        if !(path.pop() && path.pop()) {
                            anyhow::bail!("Unable to find a \"mdoc.toml\" file.")
                        }
                    }
                };

                (Config::default(), load_chapters(root))
            }
        };

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
    WalkBuilder::new(path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| Chapter::load(e.path()))
        .filter_map(|e| e.map_err(|err| warn!("{}", err)).ok())
        .collect()
}
