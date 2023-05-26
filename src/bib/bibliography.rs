use std::io;

use hayagriva::{io::from_biblatex_str, Entry};

use crate::walk::Walker;

pub fn get_bib_entries() -> io::Result<Vec<Entry>> {
    let bibtex_content = Walker::new(".")?
        .max_nesting(5)
        .filter_extensions(&["bib", "bibtex"])
        .map(std::fs::read_to_string)
        .collect::<Result<String, io::Error>>()?;

    // TODO: Handle error(s)
    let entries = from_biblatex_str(&bibtex_content).unwrap();
    Ok(entries)
}
