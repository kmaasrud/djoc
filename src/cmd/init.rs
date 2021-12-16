use anyhow::{Context, Result};
use mdoc::{
    utils::{get_author_name, write_file},
    CONFIG_FILE, SRC_DIR,
};
use std::path::PathBuf;

/// Initializes a document in the path provided. Defaults to the current directory if no path is
/// provided.
pub fn init(path: Option<PathBuf>) -> Result<()> {
    // Use path argument, or default from current directory
    let root = path.unwrap_or_else(|| PathBuf::from("."));

    // Recursively create all directories
    std::fs::create_dir_all(&root.join(SRC_DIR))
        .context("Failed at creating the directory structure.")?;

    // Make default config
    let mut config = String::new();
    config.push_str(r#"# This is the configuration file of your document.
# It is used to specify metadata, build instructions, styling and more."#);
    config.push_str("\ntitle = \"Document title\"");

    // Add author name from Git if available
    if let Some(author) = get_author_name() {
        config.push_str(&format!("\nauthors = [\"{}\"]", author))
    }

    config.push_str("\n# For more options, visit https://kmaasrud.com/mdoc/config");

    // Write to file
    write_file(&root.join(CONFIG_FILE), config.as_bytes())
        .context("Could not write configuration to file.")?;

    mdoc::success!("Created a new document in {:?}.", root);

    Ok(())
}
