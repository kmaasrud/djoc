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

const PREAMBLE: &str = r#"\PassOptionsToPackage{unicode}{hyperref}
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
        buf.write_all(PREAMBLE.as_bytes())?;

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

    pub fn to_html_bytes(&self) -> Vec<u8> {
        self.chapters
            .par_iter()
            .map(|ch| {
                let mut buf = Vec::new();
                ch.write_html(&mut buf).unwrap();
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
