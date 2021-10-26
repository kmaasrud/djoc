use std::path::PathBuf;

#[allow(dead_code)]
pub struct DocumentBuilder {
    root: PathBuf,
    config: PathBuf,
}

impl DocumentBuilder {
    pub fn new(root: impl Into<PathBuf> + Clone) -> DocumentBuilder {
        DocumentBuilder {
            root: root.clone().into(),
            config: root.into().join("mdoc.toml"),
        }
    }
}
