use anyhow::Result;
use crate::utils::{data_dir, write_file};
use std::path::PathBuf;

const CROSSREF: &[u8] = include_bytes!("crossref.lua");

pub(crate) fn get_filters() -> Result<PathBuf> {
    let path = data_dir().join("filters/crossref.lua");
    if path.exists() {
        Ok(path)
    } else {
        write_file(&path, CROSSREF)?;
        Ok(path)
    }
}
