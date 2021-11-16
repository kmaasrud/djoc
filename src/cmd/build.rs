use anyhow::{bail, Context, Result};
use mdoc::{utils::write_file, DocumentBuilder};
use std::path::{Path, PathBuf};

pub fn build(file: Option<PathBuf>) -> Result<()> {
    let doc = match file {
        Some(path) => DocumentBuilder::new()
            .source(path)
            .build()?,
        None => bail!("Can't load project yet"),
    };

    let pdf_data = doc.build()?;

    write_file(&Path::new("main.pdf"), &pdf_data).context("Could not write to PDF file")?;

    Ok(())
}
