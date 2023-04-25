use std::fs;

use anyhow::Result;
use djoc::manifest::Manifest;

/// Builds a document.
pub fn build() -> Result<()> {
    std::fs::read_dir(std::env::current_dir()?)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.extension()
                .map(|ext| ext.to_string_lossy() == "toml")
                .unwrap_or_default()
        })
        // NOTE: A TOML file must be UTF-8, but should we perhaps throw an error if any file is not
        // UTF-8? That might be less confusing for users.
        .filter_map(|path| toml::from_str(&fs::read_to_string(path).ok()?).ok())
        .try_for_each(|manifest: Manifest| manifest.execute())?;

    Ok(())
}
