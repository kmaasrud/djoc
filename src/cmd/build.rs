use anyhow::{Context, Result};
use mdoc::{utils::write_file, DocumentBuilder};
use std::path::{Path, PathBuf};

pub fn build(file: Option<PathBuf>) -> Result<()> {
    let builder = DocumentBuilder::new();
    let doc = match file {
        Some(path) => builder.source(path).build()?,
        None => builder.build()?,
    };

    let pdf_data = doc.build()?;

    write_file(&Path::new("main.pdf"), &pdf_data).context("Could not write to PDF file")?;

    Ok(())
}
