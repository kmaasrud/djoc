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
        .try_for_each(|path| {
            let manifest: Manifest = toml::from_str(&fs::read_to_string(path)?)?;
            manifest.execute()?;
            Ok(())
        })
}
