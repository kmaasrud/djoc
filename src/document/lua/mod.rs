use crate::utils::{data_dir, write_file};
use anyhow::Result;
use std::path::PathBuf;

const ABSTRACT: &[u8] = include_bytes!("abstract.lua");
const CROSSREF: &[u8] = include_bytes!("crossref.lua");

pub(crate) fn get_filters() -> Result<Vec<PathBuf>> {
    let mut paths = vec![];
    for (data, fname) in [(ABSTRACT, "abstract.lua"), (CROSSREF, "crossref.lua")] {
        let path = data_dir().join("filters").join(fname);
        if !path.exists() {
            write_file(&path, data)?;
        }
        paths.push(path);
    }
    Ok(paths)
}
