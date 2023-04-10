use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs,
    io::{self, Write},
    path::Path,
    time::SystemTime,
};

use jotdown::{Parser, Render};
use rayon::prelude::*;

use super::Builder;
use crate::{latex, Document};

impl Builder {
    pub fn write_pdf<W: Write>(&self, document: &Document, mut w: W) -> Result<(), PdfError> {
        let filename = document.filename();
        let build_root = Path::new("build").join(&filename);
        fs::create_dir_all(&build_root).map_err(|e| PdfError {
            document_name: Some(document.title.clone()),
            kind: PdfErrorKind::CreateDir(e),
        })?;

        let mut status = crate::log::LoggingStatusBackend;
        let config = tectonic::config::PersistentConfig::default();
        let bundle = config
            .default_bundle(false, &mut status)
            .map_err(|e| PdfError::from(e).document_name(&document.title))?;

        let format_cache_path = config
            .format_cache_path()
            .map_err(|e| PdfError::from(e).document_name(&document.title))?;

        let mut bytes = Vec::new();
        self.write_latex(document, &mut bytes)?;

        let mut files = {
            let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
            sb.bundle(bundle)
                .primary_input_buffer(&bytes)
                .filesystem_root(&build_root)
                .keep_intermediates(true)
                .keep_logs(true)
                .tex_input_name(&format!("{filename}.tex"))
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .output_dir(&build_root)
                .build_date(SystemTime::now());

            let mut sess = sb
                .create(&mut status)
                .map_err(|e| PdfError::from(e).document_name(&document.title))?;

            sess.run(&mut status)
                .map_err(|e| PdfError::from(e).document_name(&document.title))?;

            sess.into_file_data()
        };

        match files.remove(&format!("{filename}.pdf")) {
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

    pub fn write_latex<W: Write>(&self, document: &Document, mut w: W) -> Result<(), PdfError> {
        writeln!(w, r"\documentclass{{{}}}", document.document_type.as_ref())?;
        for package in DEFAULT_PACKAGES {
            writeln!(w, r"\usepackage{{{package}}}")?;
        }

        w.write_all(DEFAULT_PREAMBLE)?;

        let locale = document
            .locale
            .split_once('_')
            .map(|(s, _)| s)
            .unwrap_or(&document.locale);
        writeln!(w, r"\setdefaultlanguage{{{locale}}}",)?;

        write!(w, r"\title{{")?;
        latex::Renderer::default()
            .write(Parser::new(&document.title), &mut w)
            .unwrap();
        writeln!(w, "}}")?;

        if let Some(date) = document.formatted_date() {
            writeln!(w, r"\date{{{date}}}")?;
        } else {
            writeln!(w, r"\predate{{}}\date{{}}\postdate{{}}")?;
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

        let content: String = document
            .texts
            .par_iter()
            .fold_with(String::new(), |mut buf, text| {
                latex::Renderer::default()
                    .number_sections(self.number_sections)
                    .push(Parser::new(text), &mut buf)
                    .unwrap();
                buf
            })
            .collect();

        writeln!(w, "\\begin{{document}}\n{content}\n\\end{{document}}")?;

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct PdfError {
    pub document_name: Option<String>,
    pub kind: PdfErrorKind,
}

impl PdfError {
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
            PdfErrorKind::Tectonic(e) => write!(f, "tectonic error: {}", e),
            PdfErrorKind::Io(e) => write!(f, "io error: {}", e),
            PdfErrorKind::CreateDir(e) => write!(f, "failed to create directory: {}", e),
            PdfErrorKind::NoPdfCreated => write!(f, "engine finished, but no pdf was created"),
        }
    }
}

impl Error for PdfError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            PdfErrorKind::Tectonic(e) => Some(e),
            PdfErrorKind::Io(e) => Some(e),
            PdfErrorKind::CreateDir(e) => Some(e),
            PdfErrorKind::NoPdfCreated => None,
        }
    }
}

unsafe impl Sync for PdfError {}

#[non_exhaustive]
#[derive(Debug)]
pub enum PdfErrorKind {
    Tectonic(tectonic::Error),
    Io(io::Error),
    CreateDir(io::Error),
    NoPdfCreated,
}

const DEFAULT_PACKAGES: [&str; 17] = [
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
