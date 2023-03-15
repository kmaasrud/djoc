use crate::{
    error::Error, manifest::DocumentManifest, utils::kebab, walk::Walker, Author, Chapter,
    DOC_DEF_FILE,
};
use anyhow::{Context, Result};
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
\PassOptionsToPackage{hyphens}{url}
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

\UseMicrotypeSet[protrusion]{basicmath} % disable protrusion for tt fonts
\setlength{\emergencystretch}{3em} % prevent overfull lines
\providecommand{\tightlist}{%
  \setlength{\itemsep}{0pt}\setlength{\parskip}{0pt}}
\setcounter{secnumdepth}{-\maxdimen} % remove section numbering
\urlstyle{same} % disable monospaced font for URLs
\hypersetup{
  hidelinks,
  pdfcreator={djoc}}
"#;

#[derive(Default)]
pub struct Document {
    pub title: String,
    chapters: Vec<Chapter>,
    authors: Vec<Author>,
    // TODO: Exchange with time module
    date: Option<Datetime>,
    _root: Option<PathBuf>,
}

impl Document {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        if path.join(DOC_DEF_FILE).exists() {
            let def: DocumentManifest =
                toml::from_str(&fs::read_to_string(path.join(DOC_DEF_FILE))?).unwrap();
            Ok(def.try_into()?)
        } else {
            let mut chapters = Vec::new();
            extend_chapters(&path, &mut chapters)?;

            Ok(Self {
                chapters,
                title: path.file_stem().unwrap().to_string_lossy().into(),
                _root: Some(path),
                ..Default::default()
            })
        }
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
        buf.write(PREAMBLE.as_bytes())?;

        if let Some(date) = self.date {
            buf.write(br"\date{")?;
            buf.write(date.to_string().as_bytes())?;
            buf.write(b"}\n")?;
        }

        for author in &self.authors {
            buf.write(br"\author{")?;
            buf.write(author.name.as_bytes())?;
            buf.write(b"}\n")?;
        }

        buf.write(b"\\begin{document}\n")?;
        buf.write(b"\\maketitle\n")?;
        let content: Vec<u8> = self
            .chapters
            .par_iter()
            .map(|ch| {
                let mut buf = Vec::new();
                ch.write_latex(&mut buf).ok();
                buf
            })
            .flatten()
            .collect();
        buf.write(&content)?;
        buf.write(b"\n\\end{document}")?;
        Ok(buf)
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
                .filesystem_root(".djoc")
                .keep_intermediates(true)
                .keep_logs(true)
                .tex_input_name(&format!("{filename}.tex"))
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .output_dir(".djoc")
                .build_date(SystemTime::now());
            // .do_not_write_output_files();

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

        if chapters.is_empty() {
            extend_chapters(".", &mut chapters)?;
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
