use anyhow::{bail, Result};
use djoc::{manifest::GlobalManifest, utils::find_root, Document, MANIFEST_FILE};
use log::{debug, info};
use std::{fs, path::Path};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build() -> Result<()> {
    let path = find_root(std::env::current_dir()?)?;
    let manifest: GlobalManifest = toml::from_str(&fs::read_to_string(path.join(MANIFEST_FILE))?)?;

    let mut docs: Vec<Document> = Vec::new();
    for doc_manifest in manifest.documents {
        docs.push(doc_manifest.try_into()?);
    }

    let format = manifest.common.output_format.unwrap_or("pdf".into());
    for doc in docs {
        let bytes = match format.as_str() {
            "html" => doc.to_html().into_bytes(),
            "tex" => doc.to_latex().into_bytes(),
            "pdf" => doc.to_pdf_bytes()?,
            _ => bail!("Unknown format `{}`", format),
        };

        let filename = Path::new(&doc.filename()).with_extension(&format);

        debug!("Writing to {filename:?}");

        fs::write(&filename, bytes)?;

        info!("{:?}, built!", filename);
    }

    Ok(())
}
