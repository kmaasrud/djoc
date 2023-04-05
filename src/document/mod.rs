mod author;
mod chapter;

pub use author::Author;
pub use chapter::Chapter;

use crate::{error::Result, latex, manifest::DocumentManifest, utils::kebab, walk::Walker};
use chrono::{NaiveDate, NaiveTime};
use jotdown::{Parser, Render};
use log::debug;
use rayon::prelude::*;
use sailfish::{runtime::Buffer, TemplateOnce};
use serde::Deserialize;
use std::{fmt::Write, fs, io, path::Path, time::SystemTime};

const DEFAULT_LOCALE: &str = "en_US";

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentType {
    #[default]
    Article,
    Report,
    Book,
}

impl AsRef<str> for DocumentType {
    fn as_ref(&self) -> &str {
        match self {
            DocumentType::Article => "article",
            DocumentType::Report => "report",
            DocumentType::Book => "book",
        }
    }
}

pub struct Document {
    title: String,
    chapters: Vec<Chapter>,
    authors: Vec<Author>,
    date: Option<NaiveDate>,
    time: Option<NaiveTime>,
    locale: String,
    document_type: DocumentType,
    number_sections: bool,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "Document".into(),
            chapters: Vec::new(),
            authors: Vec::new(),
            date: None,
            time: None,
            locale: DEFAULT_LOCALE.into(),
            document_type: Default::default(),
            number_sections: false,
        }
    }
}

impl Document {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let mut chapters = Vec::new();
        extend_chapters(&path, &mut chapters)?;

        Ok(Self {
            chapters,
            ..Default::default()
        })
    }

    pub fn from(content: String) -> Self {
        let chapter = Chapter::new(content);
        Self {
            chapters: vec![chapter],
            ..Default::default()
        }
    }

    fn formatted_date(&self) -> Option<String> {
        self.locale
            .as_str()
            .try_into()
            .map(|locale| match (self.date, self.time) {
                (Some(date), Some(time)) => Some(format!(
                    "{} {}",
                    date.format_localized("%e %B %Y", locale),
                    time.format("%H:%M")
                )),
                (Some(date), None) => Some(date.format_localized("%e %B %Y", locale).to_string()),
                _ => None,
            })
            .ok()
            .flatten()
    }

    pub fn to_latex(&self) -> String {
        let mut title = String::new();
        latex::Renderer::default()
            .push(Parser::new(&self.title), &mut title)
            .unwrap();

        let mut buf = Buffer::new();
        let tmpl = LatexTemplate {
            title: &title,
            authors: &self.authors,
            date: self.formatted_date(),
            content: self.content_to_latex(),
            locale: &self.locale,
            document_type: self.document_type.as_ref().into(),
        };
        tmpl.render_once_to(&mut buf).unwrap();

        buf.into_string()
    }

    fn content_to_latex(&self) -> String {
        self.chapters
            .par_iter()
            // NOTE: Any formatting errors results in skipping the chapter alltogether. The
            // formatting should rarely (if ever) error, though.
            .filter_map(|ch| {
                let mut buf = String::new();

                use DocumentType::*;
                if self.chapters.len() > 1 && matches!(self.document_type, Book | Report) {
                    if let Some(ref title) = ch.title {
                        writeln!(buf, r"\chapter{{{title}}}").ok()?;
                    }
                }

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
        let build_root = Path::new("build").join(&filename);
        fs::create_dir_all(&build_root)?;

        let mut status = crate::log::LoggingStatusBackend;
        let config = tectonic::config::PersistentConfig::default();
        let bundle = config.default_bundle(false, &mut status)?;

        let format_cache_path = config.format_cache_path()?;

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

            let mut sess = sb.create(&mut status)?;

            sess.run(&mut status)?;

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
                extend_chapters(&def.path, &mut chapters)?;
            } else {
                chapters.push(def.try_into()?);
            }
        }

        Ok(Self {
            chapters,
            date: def.date.and_then(|dt| dt.date).and_then(|date| {
                NaiveDate::from_ymd_opt(date.year.into(), date.month.into(), date.day.into())
            }),
            time: def.date.and_then(|dt| dt.time).and_then(|time| {
                NaiveTime::from_hms_opt(time.hour.into(), time.minute.into(), time.second.into())
            }),
            title: def.title.to_owned(),
            authors: def.authors.into_iter().map(Into::into).collect(),
            locale: def.locale.unwrap_or(DEFAULT_LOCALE.into()),
            document_type: def.document_type,
            number_sections: def.number_sections.unwrap_or_default(),
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
    locale: &'a str,
    document_type: String,
}

#[derive(TemplateOnce)]
#[template(path = "html.stpl")]
struct HtmlTemplate<'a> {
    title: &'a str,
    content: String,
}
