use crate::utils::{data_dir, write_file};
use anyhow::Result;
use std::path::PathBuf;

const FILTERS: [(&str, &[u8]); 2] = [
    ("abstract.lua", include_bytes!("abstract.lua")),
    ("crossref.lua", include_bytes!("crossref.lua"))
];

pub(crate) fn get_filters() -> Result<Vec<PathBuf>> {
    let mut paths = vec![];
    for (fname, data) in FILTERS {
        let path = data_dir().join("filters").join(fname);
        if !path.exists() {
            write_file(&path, data)?;
        }
        paths.push(path);
    }
    Ok(paths)
}
