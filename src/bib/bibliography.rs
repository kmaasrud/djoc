use crate::{error::Result, utils::find_root};
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::{Path, PathBuf};

pub fn get_bib_files<P: AsRef<Path>>(path: Option<P>) -> Result<Vec<PathBuf>> {
    let path = path.as_ref();

    match path {
        Some(path) if path.as_ref().is_dir() => Ok(load_bib_files_from_dir(path)),

        Some(path) if path.as_ref().is_file() => Ok(vec![path.as_ref().to_owned()]),

        _ => Ok(load_bib_files_from_dir(find_root()?)),
    }
}

fn load_bib_files_from_dir<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut tb = TypesBuilder::new();
    tb.add("bibtex", "*.bib").unwrap();
    tb.add("bibtex", "*.bibtex").unwrap();
    let bib_types = tb.select("bibtex").build().unwrap();

    WalkBuilder::new(path)
        .types(bib_types)
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.into_path())
        .collect()
}
