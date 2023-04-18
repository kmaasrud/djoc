use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use anyhow::{Context, Result};
use log::info;

const CONFIG_PRE: &str = r#"# This is your project's manifest file. It is where you will configure your
# document(s) and how they should be built.
"#;
const CONFIG_POST: &str = r#""#;

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
    fs::write(root.join("src").join("main.dj"), b"")?;

    let mut config = File::create(root.join("document.toml"))?;

    writeln!(config, "{CONFIG_PRE}")?;

    writeln!(config, "outputs = [\"pdf\"]")?;
    writeln!(config, "add-title = true")?;
    writeln!(config, "build-dir = \"build\"")?;
    writeln!(config)?;

    writeln!(config, "[[document]]")?;
    writeln!(config, "title = \"{title}\"")?;

    if let Some(author) = get_author_name() {
        writeln!(config, "authors = [\"{author}\"]")?;
    }

    let today = chrono::Local::now().date_naive().format("%Y-%m-%d");
    writeln!(config, "date = {today}")?;

    writeln!(config, "texts = [\"src/main.dj\"]")?;

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
