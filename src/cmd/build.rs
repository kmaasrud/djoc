use anyhow::{bail, Context, Result};
use mdoc::{utils::write_file, DocumentBuilder};
use std::path::{Path, PathBuf};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build(path: Option<PathBuf>, output_type: Option<String>) -> Result<()> {
    // Initialize Document
    let builder = DocumentBuilder::new();
    let mut doc = match path {
        Some(path) => builder.source(path).build()?,
        None => builder.build()?,
    };

    if let Some(output_type) = output_type {
        doc.config.build.output = output_type;
    }

    let (data, extension) = match doc.config.build.output.as_str() {
        "html" => (doc.html_bytes()?, "html"),
        "latex" | "tex" => (doc.latex_bytes()?, "tex"),
        "pdf" => (doc.pdf_bytes()?, "pdf"),
        other => bail!("Unknown output type \"{}\"", other),
    };

    let filename = Path::new(&doc.config.filename()).with_extension(extension);

    // Write data to file
    write_file(&filename, &data).context("Could not write to PDF file")?;

    mdoc::success!("{:?}, built!", filename);

    Ok(())
}
