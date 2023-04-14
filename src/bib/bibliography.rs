use std::{io, path::Path};

use hayagriva::Entry;

use crate::{utils::find_root, walk::Walker};

pub fn get_bib_entries<P: AsRef<Path>>(path: Option<P>) -> io::Result<Vec<Entry>> {
    let bibtex_content = match path {
        Some(path) => Walker::new(path)?.filter_extensions(&["bib", "bibtex"]),
        _ => Walker::new(".")?.filter_extensions(&["bib", "bibtex"]),
    }
    .map(std::fs::read_to_string)
    .collect::<Result<String, io::Error>>()?;

    // TODO: Handle error(s)
    Ok(hayagriva::io::from_biblatex_str(&bibtex_content).unwrap())
}
