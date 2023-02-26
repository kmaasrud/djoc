use anyhow::{bail, Result};
use djoc::Document;
use log::{debug, info};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build(path: PathBuf, output: String) -> Result<()> {
    // Initialize Document
    let doc = Document::from_path(path)?;

    // Find the file extension (`latex` is an alias to `tex`)
    let output = output.replace("latex", "tex");

    // Produce the bytes according to the output type
    let bytes = match output.as_str() {
        "html" => doc.to_html_bytes(),
        "tex" => doc.to_latex_bytes(),
        "pdf" => doc.to_pdf_bytes()?,
        _ => bail!("Unknown output type \"{}\"", output),
    };

    // Create filename
    let filename = Path::new(&doc.filename()).with_extension(output);

    debug!("Writing to {filename:?}");

    // Write bytes to file
    fs::write(&filename, bytes)?;

    info!("{:?}, built!", filename);

    Ok(())
}
