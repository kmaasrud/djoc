use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{bail, Result};
use djoc::{Builder, Document};
use log::debug;

/// Builds a document. If no path is provided, searches up the filetree for a
/// document to build.
pub fn compile(
    path: Option<PathBuf>,
    format: String,
    output: Option<PathBuf>,
    number_sections: bool,
) -> Result<()> {
    let mut builder = Builder::default();
    builder.number_sections(number_sections);
    let doc = match path {
        Some(path) => Document::from_path(path)?,
        None => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s)?;
            Document::from(s)
        }
    };

    let format = format.replace("latex", "tex");
    if let Some(output) = output {
        let file = File::create(&output)?;
        debug!("Writing to {output:?}");
        match format.as_str() {
            #[cfg(any(feature = "html", feature = "html-wasm"))]
            "html" => builder.write_html(&doc, file)?,
            #[cfg(feature = "latex")]
            "tex" | "latex" => builder.write_latex(&doc, file)?,
            #[cfg(feature = "pdf")]
            "pdf" => builder.write_pdf(&doc, file)?,
            _ => bail!("Unknown format `{}`", format),
        };
    } else {
        let stdout = std::io::stdout();
        match format.as_str() {
            #[cfg(any(feature = "html", feature = "html-wasm"))]
            "html" => builder.write_html(&doc, stdout)?,
            #[cfg(feature = "latex")]
            "tex" | "latex" => builder.write_latex(&doc, stdout)?,
            #[cfg(feature = "pdf")]
            "pdf" => builder.write_pdf(&doc, stdout)?,
            _ => bail!("Unknown format `{}`", format),
        };
    }

    Ok(())
}
