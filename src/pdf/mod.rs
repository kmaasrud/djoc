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
    sync::{Arc, Mutex},
    time::SystemTime,
};

use hayagriva::style::{Citation, Database, ChicagoAuthorDate};
use jotdown::{Container, Event, Parser, Render};
use rayon::prelude::*;

use super::Builder;
use crate::{latex, Document};

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

    /// Build the document as LaTeX and write it to the given writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use djoc::{Builder, Document};
    ///
    /// let mut builder = Builder::default();
    /// let document = Document::from("Hello, world!".to_string());
    /// builder
    ///     .write_latex(&document, &mut std::io::stdout())
    ///     .unwrap();
    /// ```
    pub fn write_latex<'s, W: Write>(
        &self,
        document: &'s Document,
        mut w: W,
    ) -> Result<(), PdfError> {
        let mut inner = || -> Result<(), PdfError> {
            writeln!(w, r"\documentclass{{{}}}", document.document_type.as_ref())?;

            DEFAULT_PACKAGES
                .iter()
                .try_for_each(|package| writeln!(w, r"\usepackage{{{package}}}"))?;
            w.write_all(DEFAULT_PREAMBLE)?;

            let lang = self
                .locale
                .split_once('_')
                .map_or(self.locale.as_str(), |(s, _)| s);
            writeln!(w, r"\setdefaultlanguage{{{lang}}}")?;

            write!(w, r"\title{{")?;
            latex::Renderer::default().write(Parser::new(&document.title), &mut w)?;
            writeln!(w, "}}")?;

            match document.date.format_with_locale(&self.locale) {
                Some(date) => writeln!(w, r"\date{{{date}}}")?,
                None => writeln!(w, r"\predate{{}}\date{{}}\postdate{{}}")?,
            }

            if document.authors.is_empty() {
                writeln!(w, r"\preauthor{{}}\author{{}}\postauthor{{}}")?;
            }

            for author in &document.authors {
                write!(w, r"\author{{{}", author.name)?;
                if let Some(ref email) = author.email {
                    write!(w, r" \thanks{{\href{{mailto:{email}}}{{{email}}}}}")?;
                }
                writeln!(w, "}}")?;
            }

            let db = Arc::new(Mutex::new(Database::from_entries(self.bib.iter())));
            let bib_style = Arc::new(Mutex::new(ChicagoAuthorDate::new()));
            writeln!(w, r"\begin{{document}}")?;

            if self.add_title {
                writeln!(w, r"\maketitle")?;
            }

            document
                .texts
                .par_iter()
                .try_fold_with(Vec::new(), |mut buf, text| {
                    let mut cite = None;
                    let citations = |event: Event<'s>| -> Event<'s> {
                        match event {
                            Event::Start(Container::Span, ref attrs) => {
                                if let Some(key) = attrs.get("cite") {
                                    let mut db = db.lock().unwrap();
                                    // TODO: Should avoid this clone
                                    if let Some(record) =
                                        db.records.clone().get(key.to_string().as_str())
                                    {
                                        let citation = db.citation(
                                            &mut *bib_style.lock().unwrap(),
                                            &[Citation::new(record.entry, None)],
                                        );
                                        cite = Some(citation.display.value);
                                    }
                                }
                            }
                            Event::Str(ref s) if cite.is_some() && s == "@" => {
                                return Event::Str(cite.take().unwrap().into())
                            }
                            _ => {}
                        }
                        event
                    };
                    latex::Renderer::default()
                        .number_sections(self.number_sections)
                        .write(Parser::new(text).map(citations), &mut buf)?;
                    Ok(buf)
                })
                .collect::<Result<Vec<Vec<u8>>, PdfError>>()?
                .into_iter()
                .try_for_each(|s| w.write_all(&s))?;

            writeln!(w, r"\begin{{itemize}}")?;
            for reference in db.lock().unwrap().bibliography(&ChicagoAuthorDate::new(), None) {
                write!(w, r"\item")?;
                if let Some(prefix) = reference.prefix {
                    write!(w, r"[{}]", prefix.value.replace('[', "\\["))?;
                }
                writeln!(w, r" {}", reference.display.value)?;
            }
            writeln!(w, r"\end{{itemize}}")?;

            writeln!(w, r"\end{{document}}")?;

            Ok(())
        };

        inner().map_err(|e| e.document_name(&document.title))
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

impl Display for PdfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.document_name {
            write!(f, "{name} - ")?;
        }

        match &self.kind {
            PdfErrorKind::Tectonic(_) => write!(f, "tectonic errored during pdf build"),
            PdfErrorKind::Io(e) => write!(f, "io error: {e}"),
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
    CreateDir { path: PathBuf, source: io::Error },
    NoPdfCreated,
}

const DEFAULT_PACKAGES: [&str; 18] = [
    "amsmath",
    "authblk",
    "bookmark",
    "graphicx",
    "hyperref",
    "microtype",
    "parskip",
    "soul",
    "titling",
    "upquote",
    "xurl",
    "xcolor",
    "lmodern",
    "unicode-math",
    "polyglossia",
    "pifont",
    "enumitem",
    "subcaption",
];

const DEFAULT_PREAMBLE: &[u8] = br#"
\defaultfontfeatures{Scale=MatchLowercase}
\defaultfontfeatures[\rmfamily]{Ligatures=TeX,Scale=1}

% Task lists
\newcommand{\checkbox}{\text{\fboxsep=-.15pt\fbox{\rule{0pt}{1.5ex}\rule{1.5ex}{0pt}}}}
\newcommand{\done}{\rlap{\checkbox}{\raisebox{2pt}{\large\hspace{1pt}\ding{51}}}\hspace{-2.5pt}}
\newlist{tasklist}{itemize}{2}
\setlist[tasklist]{label=\checkbox}

% Other settings
\UseMicrotypeSet[protrusion]{basicmath} % disable protrusion for tt fonts
\setlength{\emergencystretch}{3em} % prevent overfull lines
\providecommand{\tightlist}{%
  \setlength{\itemsep}{0pt}\setlength{\parskip}{0pt}}
\urlstyle{same} % disable monospaced font for URLs
\hypersetup{
  colorlinks=true,
  allcolors=.,
  urlcolor=blue,
  linktocpage,
  pdfcreator={djoc}}
"#;
