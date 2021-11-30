use anyhow::Result;
use mdoc::{config::Config, utils::{write_file, get_author_name}, CONFIG_FILE};
use std::path::PathBuf;

pub fn init(path: Option<PathBuf>) -> Result<()> {
    let root = path.unwrap_or_else(|| PathBuf::from(".")); 
    std::fs::create_dir_all(&root)?;

    let mut config = Config::default();
    if let Some(author) = get_author_name() {
        config.authors.push(author);
    }

    write_file(&root.join(CONFIG_FILE), &toml::to_vec(&config)?)?;

    Ok(())
}
