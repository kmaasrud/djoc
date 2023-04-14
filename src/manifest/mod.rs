//! Contains the types and functions for parsing and executing a manifest file.

mod builder;
mod document;
mod serde_impls;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    path::Path,
};

pub(crate) use builder::{BuilderManifest, Output, OutputFormat};
pub(crate) use document::DocumentManifest;
use rayon::prelude::*;
use serde::Deserialize;

use crate::{builder::Builder, Document};

/// Represents a complete manifest file.
///
/// # Examples
///
/// ```
/// use djoc::Manifest;
///
/// let toml = r#"
/// [[document]]
/// title = "My Document"
/// authors = ["John Doe"]
/// output = ["pdf"]
/// "#;
/// let manifest: Manifest = toml::from_str(toml).unwrap();
/// ```
#[derive(Deserialize)]
pub struct Manifest {
    #[serde(alias = "document")]
    documents: Vec<DocumentManifest>,
    #[serde(flatten)]
    builder: BuilderManifest,
}

impl Manifest {
    /// Executes the build process as specified for all documents defined in the
    /// manifest.
    pub fn execute(self) -> Result<(), ExecutionError> {
        let builder_manifest = self.builder;
        self.documents
            .into_par_iter()
            .try_for_each(|manifest| -> Result<(), ExecutionError> {
                let builder_manifest = builder_manifest.merge(manifest.builder.to_owned());
                let builder = Builder::from_manifest(&builder_manifest);

                let document = Document::from_manifest(&manifest)?;

                for output in builder_manifest.outputs {
                    let path = Path::new(&output.name.unwrap_or(document.filename()))
                        .with_extension(output.format.as_ref());
                    let file = File::create(path)?;
                    match output.format {
                        OutputFormat::Pdf => builder.write_pdf(&document, file)?,
                        OutputFormat::Latex => builder.write_latex(&document, file)?,
                        OutputFormat::Html => builder.write_html(&document, file)?,
                    };
                }

                Ok(())
            })
    }
}

/// Represents an error that occurred during the execution of a manifest.
#[derive(Debug)]
pub enum ExecutionError {
    Pdf(crate::pdf::PdfError),
    Html(crate::html::HtmlError),
    Io(std::io::Error),
}

impl From<crate::pdf::PdfError> for ExecutionError {
    fn from(e: crate::pdf::PdfError) -> Self {
        Self::Pdf(e)
    }
}

impl From<crate::html::HtmlError> for ExecutionError {
    fn from(e: crate::html::HtmlError) -> Self {
        Self::Html(e)
    }
}

impl From<std::io::Error> for ExecutionError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pdf(e) => write!(f, "failed during pdf build: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Html(e) => write!(f, "failed during html build: {e}"),
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Pdf(e) => Some(e),
            Self::Io(e) => Some(e),
            Self::Html(e) => Some(e),
        }
    }
}
