//! PDF output functionality for djoc.
//!
//! This module only contains the error types for PDF output and provides the
//! [`Builder::write_pdf`] and [`Builder::write_latex`] methods.

mod status;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs,
    io::{self, Write},
    path::PathBuf,
    time::SystemTime,
};

use super::Builder;
use crate::{latex::LatexError, Document};

impl Builder {
    /// Build the document as PDF and write it to the given writer.
    ///
    /// # Examples
    ///
    /// *Example removed because it is not possible to test it on CI.
    /// Works the same as [`Builder::write_latex`].*
    pub fn write_pdf<W: Write>(&self, document: &Document, mut w: W) -> Result<(), PdfError> {
        let with_name = |e| PdfError::from(e).document_name(&document.title);
        let filename = document.filename();

        let mut status = status::LoggingStatusBackend;
        let config = tectonic::config::PersistentConfig::default();
        let bundle = config
            .default_bundle(false, &mut status)
            .map_err(with_name)?;

        let format_cache_path = config.format_cache_path().map_err(with_name)?;

        let mut bytes = Vec::new();
        self.write_latex(document, &mut bytes)?;

        let files = {
            let mut sb = tectonic::driver::ProcessingSessionBuilder::default();

            sb.bundle(bundle)
                .primary_input_buffer(&bytes)
                .keep_intermediates(true)
                .keep_logs(true)
                .tex_input_name(&format!("{filename}.tex"))
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .build_date(SystemTime::now());

            if let Some(ref build_dir) = self.build_dir {
                let build_dir = build_dir.join(&filename);
                sb.filesystem_root(&build_dir).output_dir(&build_dir);
                fs::create_dir_all(&build_dir).map_err(|e| PdfError {
                    document_name: Some(document.title.clone()),
                    kind: PdfErrorKind::CreateDir {
                        path: build_dir,
                        source: e,
                    },
                })?;
            } else {
                sb.do_not_write_output_files();
            }

            let mut sess = sb.create(&mut status).map_err(with_name)?;

            sess.run(&mut status).map_err(with_name)?;

            sess.into_file_data()
        };

        match files.get(&format!("{filename}.pdf")) {
            Some(file) => w.write_all(&file.data)?,
            None => {
                return Err(PdfError {
                    document_name: Some(document.title.clone()),
                    kind: PdfErrorKind::NoPdfCreated,
                })
            }
        }
        Ok(())
    }
}

/// An error that can occur when building a PDF.
#[non_exhaustive]
#[derive(Debug)]
pub struct PdfError {
    /// The title of the document that caused the error.
    pub document_name: Option<String>,
    /// The kind of error that occurred.
    pub kind: PdfErrorKind,
}

impl PdfError {
    /// Set the name of the document that caused the error.
    #[must_use]
    pub fn document_name(self, document_name: &str) -> Self {
        Self {
            document_name: Some(document_name.to_string()),
            ..self
        }
    }
}

impl From<io::Error> for PdfError {
    fn from(e: io::Error) -> Self {
        Self {
            document_name: None,
            kind: PdfErrorKind::Io(e),
        }
    }
}

impl From<tectonic::Error> for PdfError {
    fn from(e: tectonic::Error) -> Self {
        Self {
            document_name: None,
            kind: PdfErrorKind::Tectonic(e),
        }
    }
}

impl From<LatexError> for PdfError {
    fn from(e: LatexError) -> Self {
        Self {
            document_name: None,
            kind: PdfErrorKind::Latex(e),
        }
    }
}

impl Display for PdfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.document_name {
            write!(f, "{name} - ")?;
        }

        match &self.kind {
            PdfErrorKind::Tectonic(_) => write!(f, "tectonic errored during pdf build"),
            PdfErrorKind::Io(e) => write!(f, "io error: {e}"),
            PdfErrorKind::Latex(e) => write!(f, "djot to latex error: {e}"),
            PdfErrorKind::CreateDir { path, .. } => {
                write!(f, "failed to create directory {path:?}")
            }
            PdfErrorKind::NoPdfCreated => write!(f, "engine finished, but no pdf was created"),
        }
    }
}

impl Error for PdfError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            PdfErrorKind::Tectonic(source) => Some(source),
            PdfErrorKind::Io(source) => Some(source),
            PdfErrorKind::Latex(source) => Some(source),
            PdfErrorKind::CreateDir { source, .. } => Some(source),
            PdfErrorKind::NoPdfCreated => None,
        }
    }
}

unsafe impl Sync for PdfError {}

/// The kind of error that can occur when building a PDF.
#[non_exhaustive]
#[derive(Debug)]
pub enum PdfErrorKind {
    Tectonic(tectonic::Error),
    Io(io::Error),
    Latex(LatexError),
    CreateDir { path: PathBuf, source: io::Error },
    NoPdfCreated,
}
