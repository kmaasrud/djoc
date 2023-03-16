use crate::{
    error::Error, latex, manifest::DocumentManifest, utils::kebab, walk::Walker, Author, Chapter,
};
use anyhow::{Context, Result};
use jotdown::{Parser, Render};
use log::debug;
use rayon::prelude::*;
use sailfish::{runtime::Buffer, TemplateOnce};
use std::{fs, io, path::Path, time::SystemTime};
use toml::value::Datetime;

#[derive(Default)]
pub struct Document {
    title: String,
    chapters: Vec<Chapter>,
    authors: Vec<Author>,
    date: Option<Datetime>,
}

impl Document {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let mut chapters = Vec::new();
        extend_chapters(&path, &mut chapters)?;

        Ok(Self {
            chapters,
            title: path.file_stem().unwrap().to_string_lossy().into(),
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

    pub fn to_latex(&self) -> String {
        let mut title = String::new();
        latex::Renderer::default()
            .push(Parser::new(&self.title), &mut title)
            .unwrap();

        let mut buf = Buffer::new();
        let tmpl = LatexTemplate {
            title: title.trim(),
            authors: &self.authors,
            date: self.date.map(|dt| dt.to_string()),
            content: self.content_to_latex(),
        };
        tmpl.render_once_to(&mut buf).unwrap();

        buf.into_string()
    }

    fn content_to_latex(&self) -> String {
        self.chapters
            .par_iter()
            .filter_map(|ch| {
                let mut buf = String::new();
                ch.write_latex(&mut buf).ok()?;
                Some(buf)
            })
            .collect()
    }

    pub fn to_html(&self) -> String {
        let mut buf = Buffer::new();
        let tmpl = HtmlTemplate {
            title: &self.title,
            content: self.content_to_html(),
        };
        tmpl.render_once_to(&mut buf).unwrap();

        buf.into_string()
    }

    fn content_to_html(&self) -> String {
        self.chapters
            .par_iter()
            .map(|ch| {
                let mut buf = String::new();
                ch.write_html(&mut buf).ok();
                buf
            })
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
                .primary_input_buffer(self.to_latex().as_bytes())
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

#[derive(TemplateOnce)]
#[template(path = "latex.stpl")]
struct LatexTemplate<'a> {
    title: &'a str,
    authors: &'a [Author],
    date: Option<String>,
    content: String,
}

#[derive(TemplateOnce)]
#[template(path = "html.stpl")]
struct HtmlTemplate<'a> {
    title: &'a str,
    content: String,
}
