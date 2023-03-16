use crate::{
    error::Error, latex, manifest::DocumentManifest, utils::kebab, walk::Walker, Author, Chapter,
};
use anyhow::{Context, Result};
use jotdown::{Parser, Render};
use log::debug;
use rayon::prelude::*;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};
use toml::value::Datetime;

#[derive(Default)]
pub struct Document {
    pub title: String,
    chapters: Vec<Chapter>,
    authors: Vec<Author>,
    date: Option<Datetime>,
    _root: Option<PathBuf>,
}

impl Document {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let mut chapters = Vec::new();
        extend_chapters(&path, &mut chapters)?;

        Ok(Self {
            chapters,
            title: path.file_stem().unwrap().to_string_lossy().into(),
            _root: Some(path),
            ..Default::default()
        })
    }

    pub fn from(content: impl ToString) -> Self {
        let chapter = Chapter::new("title", content);
        Self {
            title: chapter.title.clone(),
            chapters: vec![chapter],
            ..Default::default()
        }
    }

    pub fn to_latex(&self) -> Result<String> {
        Ok(String::from_utf8(self.to_latex_bytes()?)?)
    }

    pub fn to_latex_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write_all(PREAMBLE_LATEX)?;

        buf.write_all(br"\title{")?;
        latex::Renderer::default().write(Parser::new(&self.title), &mut buf)?;
        writeln!(buf, "}}")?;

        if let Some(date) = self.date {
            writeln!(buf, "\\date{{{}}}", date)?;
        } else {
            writeln!(buf, "\\predate{{}}\n\\date{{}}\n\\postdate{{}}")?;
        }

        if self.authors.is_empty() {
            writeln!(buf, "\\preauthor{{}}\n\\postauthor{{}}\n")?;
        }
        for author in &self.authors {
            writeln!(buf, "\\author{{{}}}", author.name)?;
        }

        buf.write_all(b"\\begin{document}\n")?;
        buf.write_all(b"\\maketitle\n")?;
        buf.write_all(&self.content_to_latex())?;
        buf.write_all(b"\n\\end{document}")?;
        Ok(buf)
    }

    fn content_to_latex(&self) -> Vec<u8> {
        self.chapters
            .par_iter()
            .filter_map(|ch| {
                let mut buf = Vec::new();
                ch.write_latex(&mut buf).ok()?;
                Some(buf)
            })
            .flatten()
            .collect()
    }

    pub fn to_html_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        buf.write_all(PREAMBLE_HTML)?;
        writeln!(buf, "<title>{}</title>", self.title)?;
        writeln!(buf, "</head>\n<body>")?;
        buf.write_all(&self.content_to_html())?;
        buf.write_all(b"\n</body>\n</html>")?;
        Ok(buf)
    }

    fn content_to_html(&self) -> Vec<u8> {
        self.chapters
            .par_iter()
            .map(|ch| {
                let mut buf = Vec::new();
                ch.write_html(&mut buf).ok();
                buf
            })
            .flatten()
            .collect()
    }

    pub fn filename(&self) -> String {
        kebab(&self.title)
    }

    pub fn to_pdf_bytes(&self) -> Result<Vec<u8>> {
        let filename = self.filename();
        let build_root = Path::new(".djoc").join(&filename);
        fs::create_dir_all(&build_root)?;

        let mut status = crate::log::DjocTectonicStatusBackend { tidy_logs: true };

        let config = tectonic::config::PersistentConfig::default();
        let bundle = config
            .default_bundle(false, &mut status)
            .map_err(Error::Tectonic)
            .context("Failed to load the default resource bundle.")?;

        let format_cache_path = config
            .format_cache_path()
            .map_err(Error::Tectonic)
            .context("Failed to set up the format cache.")?;

        let mut files = {
            let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
            sb.bundle(bundle)
                .primary_input_buffer(&self.to_latex_bytes()?)
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
                .map_err(Error::Tectonic)
                .context("Failed to initialize the LaTeX processing session.")?;

            sess.run(&mut status)
                .map_err(Error::Tectonic)
                .context("The LaTeX engine failed.")?;

            sess.into_file_data()
        };

        match files.remove(&format!("{filename}.pdf")) {
            Some(file) => Ok(file.data),
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                "LaTeX didn't report failure, but no PDF was created.",
            )
            .into()),
        }
    }
}

impl TryFrom<DocumentManifest> for Document {
    type Error = io::Error;

    fn try_from(def: DocumentManifest) -> Result<Self, Self::Error> {
        let mut chapters = Vec::new();
        for def in def.chapters {
            if def.path.is_dir() {
                extend_chapters(def.path, &mut chapters)?;
            } else {
                chapters.push(def.try_into()?);
            }
        }

        let mut authors = Vec::new();
        for def in def.authors {
            authors.push(def.into());
        }

        Ok(Self {
            chapters,
            title: def.title,
            date: def.date,
            authors,
            ..Default::default()
        })
    }
}

fn extend_chapters(path: impl AsRef<Path>, chapters: &mut Vec<Chapter>) -> io::Result<()> {
    for path in Walker::new(path)?.filter_extensions(&["dj"]) {
        debug!("Loading chapter from {path:?}...");
        chapters.push(Chapter::from_path(path)?);
    }

    Ok(())
}

const PREAMBLE_LATEX: &[u8] = br#"\PassOptionsToPackage{unicode}{hyperref}
\documentclass[]{article}
\usepackage{lmodern}
\usepackage{unicode-math}
\defaultfontfeatures{Scale=MatchLowercase}
\defaultfontfeatures[\rmfamily]{Ligatures=TeX,Scale=1}
\usepackage{amsmath}
\usepackage{authblk}
\usepackage{upquote}
\usepackage[]{microtype}
\usepackage{bookmark}
\usepackage{hyperref}
\usepackage{xurl}
\usepackage{parskip}
\usepackage{xcolor}
\usepackage{soul}
\usepackage{graphicx}
\usepackage{titling}

\UseMicrotypeSet[protrusion]{basicmath} % disable protrusion for tt fonts
\setlength{\emergencystretch}{3em} % prevent overfull lines
\providecommand{\tightlist}{%
  \setlength{\itemsep}{0pt}\setlength{\parskip}{0pt}}
\setcounter{secnumdepth}{-\maxdimen} % remove section numbering
\urlstyle{same} % disable monospaced font for URLs
\hypersetup{
  hidelinks,
  pdfcreator={djoc}}

% Task lists
\usepackage{pifont}
\newcommand{\checkbox}{\text{\fboxsep=-.15pt\fbox{\rule{0pt}{1.5ex}\rule{1.5ex}{0pt}}}}
\newcommand{\done}{\rlap{\checkbox}{\raisebox{2pt}{\large\hspace{1pt}\ding{51}}}\hspace{-2.5pt}}
\usepackage{enumitem}
\newlist{tasklist}{itemize}{2}
\setlist[tasklist]{label=\checkbox}
"#;

const PREAMBLE_HTML: &[u8] = br#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<style>
*,
*::before,
*::after {
  box-sizing: border-box;
}
body { margin: 0; }
@media (prefers-reduced-motion: no-preference) {
  html {
    scroll-behavior: smooth;
  }
}
html {
  max-width: 80ch;
  overflow-x: hidden;
  padding: 3em 1em;
  margin: auto;
  line-height: 1.5;
  font-size: 1.2em;
  color: #1a1a1a;
  text-rendering: optimizeLegibility;
  hyphens: auto;
  overflow-wrap: break-word;
  font-kerning: normal;
}
article > * + * {
  margin-top: 1em;
}
h1 {
  font-size: 2rem;
  line-height: 3.25rem;
  margin-bottom: 1rem;
}

h2 {
  font-size: 1.7rem;
  line-height: 2rem;
  margin-top: 3rem;
}

h3 {
  font-size: 1.4rem;
  margin-top: 2.5rem;
}

h4 {
  font-size: 1.2rem;
  margin-top: 2rem;
}

h5 {
  font-size: 1rem;
  margin-top: 1.8rem;
}

h6 {
  font-size: 1rem;
  font-style: italic;
  font-weight: normal;
  margin-top: 2.5rem;
}

h3,
h4,
h5,
h6 {
  line-height: 1.625rem;
}

h1 + h2 {
  margin-top: 1.625rem;
}

h2 + h3,
h3 + h4,
h4 + h5 {
  margin-top: 0.8rem;
}

h5 + h6 {
  margin-top: -0.8rem;
}

h2,
h3,
h4,
h5,
h6 {
  margin-bottom: 0.8rem;
}
p,ul,ol {
  font-family: sans-serif;
}
a { color: #1a1a1a; }
a:visited { color: #414141; }
img {
  max-width: 100%;
  height: auto;
  display: block;
  margin: auto;
}
code {
  font-family: monospace;
  font-size: .9em;
}
pre {
  padding: 1rem 1.4rem;
  max-width: 100%;
  overflow: auto;
  border-radius: 4px;
  background: #eee;
}
pre code { position: relative; }
</style>
"#;
