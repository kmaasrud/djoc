use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use anyhow::{Context, Result};
use log::info;

const CONFIG_PRE: &str = r#"# This is the configuration file of your document."#;

const CONFIG_POST: &str = r#"# For more options, visit https://kmaasrud.com/djoc/config"#;

/// Initializes a document in the path provided. Defaults to the current
/// directory if no path is provided.
pub fn init(path: Option<PathBuf>) -> Result<()> {
    let (root, title) = if let Some(path) = path {
        (
            path.clone(),
            path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or("Document title".to_string()),
        )
    } else {
        (PathBuf::from("."), "Document title".to_string())
    };

    fs::create_dir_all(root.join("src")).context("Failed at creating the directory structure.")?;

    let mut config = File::create(root.join("document.toml"))?;

    writeln!(config, "{CONFIG_PRE}")?;
    writeln!(config, "[[document]]")?;
    writeln!(config, "title = \"{title}\"")?;
    if let Some(author) = get_author_name() {
        writeln!(config, "authors = [\"{author}\"]")?;
    }
    writeln!(config, "outputs = [\"pdf\"]")?;
    writeln!(config, "{CONFIG_POST}")?;

    info!("Created a new document in {:?}.", root);

    Ok(())
}

// Function fetched from https://github.com/rust-lang/mdBook/blob/master/src/cmd/init.rs
/// Obtains author name from git config file by running the `git config`
/// command.
pub fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}
