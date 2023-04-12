use std::fs;

use anyhow::Result;
use djoc::{manifest::Manifest, walk::Walker};

use crate::utils::find_root;

/// Builds a document.
pub fn build() -> Result<()> {
    Walker::new(find_root(std::env::current_dir()?))?
        .max_nesting(1)
        .filter_extensions(&["toml"])
        .try_for_each(|path| {
            let manifest: Manifest = toml::from_str(&fs::read_to_string(path)?)?;
            manifest.execute()?;
            Ok(())
        })
}
