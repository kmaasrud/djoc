use anyhow::{bail, Result};
use djoc::{manifest::GlobalManifest, utils::find_root, Document, MANIFEST_FILE};
use log::{debug, info};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build(path: PathBuf, format: String, output: Option<String>) -> Result<()> {
    let docs: Vec<Document> = match find_root(&path) {
        Ok(path) if path.join(MANIFEST_FILE).exists() => {
            let manifest: GlobalManifest =
                toml::from_str(&fs::read_to_string(path.join(MANIFEST_FILE))?)?;

            let mut docs = Vec::new();
            for doc_manifest in manifest.documents {
                docs.push(doc_manifest.try_into()?);
            }
            docs
        }
        _ => vec![Document::from_path(&path)?],
    };

    let format = format.replace("latex", "tex");
    for doc in docs {
        let bytes = match format.as_str() {
            "html" => doc.to_html_bytes(),
            "tex" => doc.to_latex_bytes()?,
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
    }

    Ok(())
}
