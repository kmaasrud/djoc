use anyhow::{Context, Result};
use mdoc::{utils::write_file, DocumentBuilder};
use std::path::{Path, PathBuf};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build(path: Option<PathBuf>) -> Result<()> {
    // Initialize Document
    let builder = DocumentBuilder::new();
    let doc = match path {
        Some(path) => builder.source(path).build()?,
        None => builder.build()?,
    };

    // Make PDF data
    let pdf_data = doc.build()?;
    let pdf_filename = Path::new(&doc.config.filename()).with_extension("pdf");

    // Write PDF data to file
    write_file(
        &pdf_filename,
        &pdf_data,
    )
    .context("Could not write to PDF file")?;

    mdoc::success!("{:?}, built!", pdf_filename);

    Ok(())
}
