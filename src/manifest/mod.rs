mod builder;
mod document;
mod serde_impls;

pub use builder::{BuilderManifest, Output, OutputFormat};
pub use document::DocumentManifest;

use crate::builder::Builder;
use crate::Document;
use rayon::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize)]
pub struct Manifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
}

impl Manifest {
    pub fn execute(self) -> Result<(), std::io::Error> {
        let builder = Builder::default();
        self.documents
            .into_par_iter()
            .try_for_each_with(builder, |builder, manifest| {
                if let Some(number_sections) = manifest.builder.number_sections {
                    builder.number_sections = number_sections;
                }

                let outputs = manifest.builder.outputs.clone();
                let document: Document = manifest.try_into()?;

                for output in outputs {
                    let path = Path::new(&output.name.unwrap_or(document.filename()))
                        .with_extension(output.format.as_ref());
                    let file = File::create(path)?;
                    match output.format {
                        OutputFormat::Pdf => builder.write_pdf(&document, file),
                        OutputFormat::Latex => builder.write_latex(&document, file),
                        OutputFormat::Html => builder.write_html(&document, file),
                    }?;
                }

                Ok(())
            })
    }
}
