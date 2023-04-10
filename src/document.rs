use crate::{error::Result, manifest::DocumentManifest, utils::kebab, walk::Walker, Author};
use chrono::{NaiveDate, NaiveTime};
use log::debug;
use serde::Deserialize;
use std::{fs, io, path::Path};

const DEFAULT_LOCALE: &str = "en_US";

#[derive(Clone, Debug, Default, Deserialize)]
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
    pub title: String,
    pub texts: Vec<String>,
    pub authors: Vec<Author>,
    date: Option<NaiveDate>,
    time: Option<NaiveTime>,
    pub locale: String,
    pub document_type: DocumentType,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "Document".into(),
            texts: Vec::new(),
            authors: Vec::new(),
            date: None,
            time: None,
            locale: DEFAULT_LOCALE.into(),
            document_type: Default::default(),
        }
    }
}

impl Document {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let mut texts = Vec::new();
        extend_texts(&path, &mut texts)?;

        Ok(Self {
            texts,
            ..Default::default()
        })
    }

    pub fn from(content: String) -> Self {
        Self {
            texts: vec![content],
            ..Default::default()
        }
    }

    pub fn formatted_date(&self) -> Option<String> {
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

    pub fn filename(&self) -> String {
        kebab(&self.title)
    }
}

impl TryFrom<DocumentManifest> for Document {
    type Error = io::Error;

    fn try_from(def: DocumentManifest) -> Result<Self, Self::Error> {
        let mut texts = Vec::new();
        for path in def.texts {
            if path.is_dir() {
                extend_texts(&path, &mut texts)?;
            } else {
                texts.push(fs::read_to_string(path)?);
            }
        }

        Ok(Self {
            texts,
            date: def.date.and_then(|dt| dt.date).and_then(|date| {
                NaiveDate::from_ymd_opt(date.year.into(), date.month.into(), date.day.into())
            }),
            time: def.date.and_then(|dt| dt.time).and_then(|time| {
                NaiveTime::from_hms_opt(time.hour.into(), time.minute.into(), time.second.into())
            }),
            title: def.title.to_owned(),
            authors: def.authors.clone().into_iter().map(Into::into).collect(),
            locale: def.locale.clone().unwrap_or(DEFAULT_LOCALE.into()),
            document_type: def.document_type,
        })
    }
}

fn extend_texts(path: impl AsRef<Path>, texts: &mut Vec<String>) -> io::Result<()> {
    for path in Walker::new(path)?.filter_extensions(&["dj"]) {
        debug!("Loading chapter from {path:?}...");
        texts.push(fs::read_to_string(path)?);
    }

    Ok(())
}