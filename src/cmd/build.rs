use anyhow::{bail, Result};
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

    // Change output type if another is supplied on the command line
    if let Some(output_type) = output_type {
        doc.config.build.output = output_type;
    }

    // Find the file extension (`latex` is an alias to `tex`)
    let extension = doc.config.build.output.replace("latex", "tex");

    // Produce the bytes according to the output type
    let bytes = match extension.as_str() {
        "html" => doc.html_bytes()?,
        "tex" => doc.latex_bytes()?,
        "pdf" => doc.pdf_bytes()?,
        _ => bail!("Unknown output type \"{}\"", extension),
    };

    // Create filename
    let filename = Path::new(&doc.config.filename()).with_extension(extension);

    // Write bytes to file
    write_file(&filename, &bytes)?;

    mdoc::success!("{:?}, built!", filename);

    Ok(())
}
