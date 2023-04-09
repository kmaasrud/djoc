use djoc::{manifest::Manifest, utils::find_root, walk::Walker};
use std::error::Error;
use std::fs;

/// Builds a document.
pub fn build() -> Result<(), Box<dyn Error>> {
    Walker::new(find_root(std::env::current_dir()?)?)?
        .max_nesting(1)
        .filter_extensions(&["toml"])
        .try_for_each(|path| {
            let manifest: Manifest = toml::from_str(&fs::read_to_string(path)?)?;
            manifest.execute()?;
            Ok(())
        })
}
