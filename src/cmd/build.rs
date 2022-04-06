use anyhow::{Context, Result};
use mdoc::{utils::write_file, DocumentBuilder};
use std::path::{Path, PathBuf};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build(path: Option<PathBuf>, tex: bool) -> Result<()> {
    // Initialize Document
    let builder = DocumentBuilder::new();
    let doc = match path {
        Some(path) => builder.source(path).build()?,
        None => builder.build()?,
    };

    let (data, filename) = if tex {
        (
            doc.latex_bytes()?,
            Path::new(&doc.config.filename()).with_extension("tex"),
        )
    } else {
        let extension = match doc.config.build.output.as_str() {
            "latex" => "tex",
            other => other,
        };

        (
            doc.build()?,
            Path::new(&doc.config.filename()).with_extension(extension),
        )
    };

    // Write data to file
    write_file(&filename, &data).context("Could not write to PDF file")?;

    mdoc::success!("{:?}, built!", filename);

    Ok(())
}
