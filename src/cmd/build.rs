use anyhow::{Result, Context, bail};
use mdoc::{Document, utils::write_file};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf}
};

pub fn build(file: Option<PathBuf>) -> Result<()> {
    let doc = match file {
        Some(path) => {
            Document::load_from_single(&path)?
        },
        None => bail!("Can't load project yet"),
    };

    let pdf_data = doc.build()?;

    write_file(&Path::new("main.pdf"), &pdf_data)
        .context("Could not write to PDF file")?;

    Ok(())
}
