use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Ease-of-use function for creating a file and writing bytes to it
pub fn write_file(path: &Path, bytes: &[u8]) -> Result<()> {
    // Ensure parent directory (if present)
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)
            .with_context(|| format!("Could not create parent directory for {:?}", path))?;
    }

    // Create file
    let mut f = File::create(path)
        .with_context(|| format!("Could not create file from path {:?}", path))?;

    // Write bytes
    f.write_all(bytes)
        .with_context(|| format!("Error when writing bytes to {:?}", path))?;

    Ok(())
}
