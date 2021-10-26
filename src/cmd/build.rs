use anyhow::{Result, Context};
use mdoc::{Document, utils::write_file};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf}
};

pub fn build(file: Option<PathBuf>) -> Result<()> {
    let content = match file {
        Some(path) => {
            let file = File::open(&path)
                .with_context(|| format!("Could not open file {:?}", path))?;

            let mut content = String::new();
            BufReader::new(&file).read_to_string(&mut content)
                .with_context(|| format!("Could not read '{:?}' to string", file))?;

            content
        },
        None => "Didn't find file".to_owned(),
    };

    let doc = Document::from(content);

    let pdf_data = doc.build()?;

    write_file(&Path::new("main.pdf"), &pdf_data)
        .context("Could not write to PDF file")?;

    Ok(())
}
