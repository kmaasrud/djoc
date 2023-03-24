use anyhow::Result;
use djoc::{
    manifest::{GlobalManifest, OutputFormat},
    utils::find_root,
    Document, MANIFEST_FILE,
};
use log::{debug, info};
use std::{fs, path::Path};

/// Builds a document. If no path is provided, searches up the filetree for a document to build.
pub fn build() -> Result<()> {
    let path = find_root(std::env::current_dir()?)?;
    let manifest: GlobalManifest = toml::from_str(&fs::read_to_string(path.join(MANIFEST_FILE))?)?;

    for doc_manifest in manifest.documents {
        let format = doc_manifest.output_format.clone().unwrap_or_default();
        let doc: Document = doc_manifest.try_into()?;

        let bytes = match format {
            OutputFormat::Html => doc.to_html().into_bytes(),
            OutputFormat::Latex => doc.to_latex().into_bytes(),
            OutputFormat::Pdf => doc.to_pdf_bytes()?,
        };

        let filename = Path::new(&doc.filename()).with_extension(format.as_ref());

        debug!("Writing to {filename:?}");

        fs::write(&filename, bytes)?;

        info!("{:?}, built!", filename);
    }

    Ok(())
}
