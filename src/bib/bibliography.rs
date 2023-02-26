use crate::{utils::find_root, walk::Walker};
use std::{
    io,
    path::{Path, PathBuf},
};

pub fn get_bib_files<P: AsRef<Path>>(path: Option<P>) -> io::Result<Vec<PathBuf>> {
    let path = path.as_ref();

    match path {
        Some(path) if path.as_ref().is_dir() => load_bib_files_from_dir(path),
        Some(path) if path.as_ref().is_file() => Ok(vec![path.as_ref().to_owned()]),
        _ => load_bib_files_from_dir(find_root().unwrap_or(".".into())),
    }
}

fn load_bib_files_from_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    Ok(Walker::new(path)?
        .filter_extensions(&["bib", "bibtex"])
        .collect())
}
