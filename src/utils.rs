use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Ease-of-use function for loading a file from a path.
pub(crate) fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut buf = String::new();

    File::open(&path)
        .with_context(|| format!("Unable to open {:?}.", path.as_ref()))?
        .read_to_string(&mut buf)
        .with_context(|| format!("Could not read file {:?}.", path.as_ref()))?;

    Ok(buf)
}

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

/// Finds the root of a MDoc document by looking for a `mdoc.toml` file.
pub fn find_root() -> Result<PathBuf> {
    let mut path: PathBuf = std::env::current_dir().unwrap();
    let look_for = Path::new("mdoc.toml");
    loop {
        path.push(look_for);
        if path.is_file() {
            path.pop();
            return Ok(path);
        }
        if !(path.pop() && path.pop()) {
            anyhow::bail!("Unable to find an \"mdoc.toml\" file.")
        }
    }
}

// Function fetched from https://github.com/rust-lang/mdBook/blob/master/src/cmd/init.rs
/// Obtains author name from git config file by running the `git config` command.
pub fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}
