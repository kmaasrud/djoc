use anyhow::{bail, Result};
use djoc::Document;
use log::{debug, info};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn compile(path: Option<PathBuf>, format: String, output: Option<String>) -> Result<()> {
    let doc = match path {
        Some(path) => Document::from_path(&path)?,
        None => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s)?;
            Document::from(s)
        }
    };

    let format = format.replace("latex", "tex");
    let bytes = match format.as_str() {
        "html" => doc.to_html().into_bytes(),
        "tex" => doc.to_latex().into_bytes(),
        "pdf" => doc.to_pdf_bytes()?,
        _ => bail!("Unknown format `{}`", format),
    };

    let filename = output
        .as_ref()
        .map(|o| o.into())
        .unwrap_or_else(|| Path::new(&doc.filename()).with_extension(&format));

    debug!("Writing to {filename:?}");

    fs::write(&filename, bytes)?;

    info!("{:?}, built!", filename);

    Ok(())
}
