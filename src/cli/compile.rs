use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use djoc::{Builder, Document};
use log::{debug, info};

/// Builds a document. If no path is provided, searches up the filetree for a
/// document to build.
pub fn compile(path: Option<PathBuf>, format: String, output: Option<String>) -> Result<()> {
    let builder = Builder::default();
    let doc = match path {
        Some(path) => Document::from_path(path)?,
        None => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s)?;
            Document::from(s)
        }
    };

    let filename = output
        .as_ref()
        .map(|o| o.into())
        .unwrap_or_else(|| Path::new(&doc.filename()).with_extension(&format));

    let file = File::create(&filename)?;

    debug!("Writing to {filename:?}");

    let format = format.replace("latex", "tex");
    match format.as_str() {
        "html" => builder.write_html(&doc, file)?,
        "tex" => builder.write_latex(&doc, file)?,
        "pdf" => builder.write_pdf(&doc, file)?,
        _ => bail!("Unknown format `{}`", format),
    };

    info!("{:?}, built!", filename);

    Ok(())
}
