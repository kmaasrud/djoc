use citeproc::prelude::*;
use serde::Deserialize;
use std::path::PathBuf;

pub fn reference(path: PathBuf) -> std::io::Result<()> {
    let file = std::fs::read_to_string(&path)?;
    let reference: References = match path.extension().and_then(|s| s.to_str()) {
        Some("toml") => toml::from_str(&file).unwrap(),
        Some("json") => serde_json::from_str(&file).unwrap(),
        _ => References::default(),
    };
    println!("{:#?}", reference.items);
    Ok(())
}

#[derive(Deserialize, Debug, Default)]
struct References {
    items: Vec<Reference>,
}
