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
    // NOTE: Allow unused variables to avoid compiler warnings when all features are
    // disabled
    #[allow(unused_variables)]
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
                        #[cfg(feature = "pdf")]
                        OutputFormat::Pdf => builder.write_pdf(&document, file)?,
                        #[cfg(feature = "latex")]
                        OutputFormat::Latex => builder.write_latex(&document, file)?,
                        #[cfg(any(feature = "html", feature = "html-wasm"))]
                        OutputFormat::Html => builder.write_html(&document, file)?,
                        OutputFormat::Unknown(format) => {
                            return Err(ExecutionError::UnknownFormat(format))
                        }
                    };
                }

                Ok(())
            })
    }
}

/// Represents an error that occurred during the execution of a manifest.
#[derive(Debug)]
pub enum ExecutionError {
    #[cfg(any(feature = "html", feature = "html-wasm"))]
    Html(crate::html::HtmlError),
    Io(std::io::Error),
    #[cfg(feature = "latex")]
    Latex(crate::latex::LatexError),
    #[cfg(feature = "pdf")]
    Pdf(crate::pdf::PdfError),
    UnknownFormat(String),
}

#[cfg(any(feature = "html", feature = "html-wasm"))]
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

#[cfg(feature = "latex")]
impl From<crate::latex::LatexError> for ExecutionError {
    fn from(e: crate::latex::LatexError) -> Self {
        Self::Latex(e)
    }
}

#[cfg(feature = "pdf")]
impl From<crate::pdf::PdfError> for ExecutionError {
    fn from(e: crate::pdf::PdfError) -> Self {
        Self::Pdf(e)
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(any(feature = "html", feature = "html-wasm"))]
            Self::Html(e) => write!(f, "failed during html build: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            #[cfg(feature = "latex")]
            Self::Latex(e) => write!(f, "failed during latex build: {e}"),
            #[cfg(feature = "pdf")]
            Self::Pdf(e) => write!(f, "failed during pdf build: {e}"),
            Self::UnknownFormat(e) => write!(f, "unknown output format: {e}"),
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            #[cfg(any(feature = "html", feature = "html-wasm"))]
            Self::Html(e) => Some(e),
            Self::Io(e) => Some(e),
            #[cfg(feature = "latex")]
            Self::Latex(e) => Some(e),
            #[cfg(feature = "pdf")]
            Self::Pdf(e) => Some(e),
            Self::UnknownFormat(_) => None,
        }
    }
}
