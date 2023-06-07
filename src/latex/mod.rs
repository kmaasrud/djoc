mod renderer;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

use jotdown::{Parser, Render};
use rayon::prelude::*;
use renderer::Renderer;

use super::Builder;
use crate::{latex, Document};

impl Builder {
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
    pub fn write_latex<W: Write>(&self, document: &Document, mut w: W) -> Result<(), LatexError> {
        let mut inner = || -> Result<(), LatexError> {
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

            writeln!(w, r"\begin{{document}}")?;

            if self.add_title {
                writeln!(w, r"\maketitle")?;
            }

            document
                .texts
                .par_iter()
                .try_fold_with(Vec::new(), |mut buf, text| {
                    latex::Renderer::default()
                        .number_sections(self.number_sections)
                        .write(Parser::new(text), &mut buf)?;
                    Ok(buf)
                })
                .collect::<Result<Vec<Vec<u8>>, LatexError>>()?
                .into_iter()
                .try_for_each(|s| w.write_all(&s))?;

            writeln!(w, r"\end{{document}}")?;

            Ok(())
        };

        inner().map_err(|e| e.document_name(&document.title))
    }
}

/// An error that can occur when building a PDF.
#[non_exhaustive]
#[derive(Debug)]
pub struct LatexError {
    /// The title of the document that caused the error.
    pub document_name: Option<String>,
    source: io::Error,
}

impl LatexError {
    /// Set the name of the document that caused the error.
    #[must_use]
    pub fn document_name(self, document_name: &str) -> Self {
        Self {
            document_name: Some(document_name.to_string()),
            ..self
        }
    }
}

impl Display for LatexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.document_name {
            Some(document_name) => write!(f, "error writing LaTeX for document {}", document_name),
            None => write!(f, "error writing LaTeX"),
        }
    }
}

impl From<io::Error> for LatexError {
    fn from(source: io::Error) -> Self {
        Self {
            document_name: None,
            source,
        }
    }
}

impl Error for LatexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

unsafe impl Sync for LatexError {}

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
