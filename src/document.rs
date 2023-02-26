use crate::{error::Error, utils::kebab, walk::Walker, Chapter};
use anyhow::{Context, Result};
use log::debug;
use rayon::prelude::*;
use std::{
    fs::{canonicalize, read_to_string},
    io,
    path::{Path, PathBuf},
    time::SystemTime,
};

type Author = String;

#[derive(Default)]
pub struct Document {
    pub title: String,
    chapters: Vec<Chapter>,
    _authors: Vec<Author>,
    _root: Option<PathBuf>,
}

impl Document {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = canonicalize(path)?;
        let mut chapters = Vec::new();

        let walker = Walker::new(path.clone())?.filter_extensions(&["dj"]);

        for path in walker {
            debug!("Loading chapter from {path:?}...");
            let content = read_to_string(&path)?;
            chapters.push(Chapter::new("title", content));
        }

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
        Ok(String::from_utf8(self.to_latex_bytes())?)
    }

    pub fn to_latex_bytes(&self) -> Vec<u8> {
        self.chapters
            .par_iter()
            .map(|ch| {
                let mut buf = Vec::new();
                ch.write_latex(&mut buf).ok();
                buf
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
                .primary_input_buffer(&self.to_latex_bytes())
                .tex_input_name(&format!("{filename}.tex"))
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .build_date(SystemTime::now())
                .do_not_write_output_files();

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
