use anyhow::{Context, Result};
use mdoc::{
    utils::{get_author_name, write_file},
    CONFIG_FILE, SRC_DIR,
};
use std::ffi::OsStr;
use std::path::PathBuf;

/// Initializes a document in the path provided. Defaults to the current directory if no path is
/// provided.
pub fn init(path: Option<PathBuf>) -> Result<()> {
    // Define root and title of document from optional path argument
    let (root, title) = if let Some(path) = path {
        (
            path.clone(),
            path.file_stem()
                .unwrap_or_else(|| OsStr::new("Document title"))
                .to_string_lossy()
                .to_string(),
        )
    } else {
        (PathBuf::from("."), "Document title".to_string())
    };

    // Recursively create all directories
    std::fs::create_dir_all(&root.join(SRC_DIR))
        .context("Failed at creating the directory structure.")?;

    // Make default config
    let mut config = String::new();
    config.push_str("# This is the configuration file of your document.\n");
    config.push_str("# It is used to specify metadata, build instructions, styling and more.\n\n");

    config.push_str(&format!("title = \"{}\"\n", title));
    config.push_str("date = \"today\"\n");

    // Add author name from Git if available
    if let Some(author) = get_author_name() {
        config.push_str(&format!("authors = [\"{}\"]\n", author))
    }

    config.push_str("\n# For more options, visit https://kmaasrud.com/mdoc/config");

    // Write to file
    write_file(&root.join(CONFIG_FILE), config.as_bytes())
        .context("Could not write configuration to file.")?;

    mdoc::success!("Created a new document in {:?}.", root);

    Ok(())
}
