mod builder;
mod document;
mod serde_impls;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    path::Path,
};

pub use builder::{BuilderManifest, Output, OutputFormat};
pub use document::DocumentManifest;
use rayon::prelude::*;
use serde::Deserialize;

use crate::{builder::Builder, Document};

#[derive(Deserialize)]
pub struct Manifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
    #[serde(flatten)]
    pub(crate) builder: BuilderManifest,
}

impl Manifest {
    pub fn execute(self) -> Result<(), ExecutionError> {
        let builder_manifest = self.builder;
        self.documents
            .into_par_iter()
            .try_for_each(|manifest| -> Result<(), ExecutionError> {
                let builder_manifest = builder_manifest.clone().merge(&manifest.builder);
                let builder = Builder::from_manifest(&builder_manifest);

                let document: Document = manifest.try_into()?;

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

#[derive(Debug)]
pub enum ExecutionError {
    Pdf(crate::builder::pdf::PdfError),
    Html(crate::builder::html::HtmlError),
    Io(std::io::Error),
}

impl From<crate::builder::pdf::PdfError> for ExecutionError {
    fn from(e: crate::builder::pdf::PdfError) -> Self {
        Self::Pdf(e)
    }
}

impl From<crate::builder::html::HtmlError> for ExecutionError {
    fn from(e: crate::builder::html::HtmlError) -> Self {
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
