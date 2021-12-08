use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::{Path, PathBuf};

pub fn get_bib_files<P: AsRef<Path>>(path: Option<P>) -> Vec<PathBuf> {
    if let Some(path) = path {
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
    } else {
        Vec::new()
    }
}
