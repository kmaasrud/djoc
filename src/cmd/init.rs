use anyhow::{Context, Result};
use mdoc::{config::Config, utils::{write_file, get_author_name}, CONFIG_FILE, SRC_DIR};
use std::path::PathBuf;

/// Initializes a document in the path provided. Defaults to the current directory if no path is
/// provided.
pub fn init(path: Option<PathBuf>) -> Result<()> {
    // Use path argument, or default from current directory
    let root = path.unwrap_or_else(|| PathBuf::from(".")); 

    // Recursively create all directories
    std::fs::create_dir_all(&root.join(SRC_DIR))
        .context("Failed at creating the directory structure.")?;

    // Make default config with author name fetched from Git
    let mut config = Config::default();
    if let Some(author) = get_author_name() {
        config.authors.push(author);
    }

    // Write config file
    write_file(
        &root.join(CONFIG_FILE),
        &toml::to_vec(&config).context("Could not serialize configuration to TOML.")?,
    ).context("Could not write configuration to file.")?;

    Ok(())
}
