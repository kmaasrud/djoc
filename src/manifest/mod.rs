use crate::builder::Builder;
use crate::Document;
use crate::{document::DocumentType, Author};
use rayon::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use toml::value::Datetime;

mod serde_impls;

#[derive(Deserialize)]
pub struct Manifest {
    #[serde(alias = "document")]
    pub documents: Vec<DocumentManifest>,
}

impl Manifest {
    pub fn execute(self) -> Result<(), Box<dyn Error>> {
        let builder = Builder::default();
        self.documents.into_par_iter().for_each(|manifest| {
            let mut builder = builder.clone();
            if let Some(number_sections) = manifest.builder.number_sections {
                builder.number_sections = number_sections;
            }

            let outputs = manifest.builder.outputs.clone();

            let document: Document = manifest.try_into().unwrap();

            for output in outputs {
                let file = File::create(
                    Path::new(&output.name.unwrap_or(document.filename()))
                        .with_extension(output.format.as_ref()),
                )
                .unwrap();
                match output.format {
                    OutputFormat::Pdf => builder.write_pdf(&document, file),
                    OutputFormat::Latex => builder.write_latex(&document, file),
                    OutputFormat::Html => builder.write_html(&document, file),
                }
                .unwrap();
            }
        });
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct BuilderManifest {
    #[serde(alias = "output")]
    pub outputs: Vec<Output>,
    pub number_sections: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DocumentManifest {
    pub title: String,
    pub date: Option<Datetime>,
    #[serde(default, alias = "author")]
    pub authors: Vec<Author>,
    #[serde(default, alias = "text")]
    pub texts: Vec<PathBuf>,
    #[serde(default, alias = "type")]
    pub document_type: DocumentType,
    pub locale: Option<String>,
    #[serde(flatten)]
    builder: BuilderManifest,
}

#[derive(Clone, Deserialize)]
pub struct Output {
    pub name: Option<String>,
    pub format: OutputFormat,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    Pdf,
    Html,
    #[serde(alias = "tex")]
    Latex,
}

impl AsRef<str> for OutputFormat {
    fn as_ref(&self) -> &str {
        match self {
            OutputFormat::Pdf => "pdf",
            OutputFormat::Html => "html",
            OutputFormat::Latex => "tex",
        }
    }
}
