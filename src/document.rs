use std::{fs, io, path::Path};

use log::debug;
use serde::Deserialize;

use crate::{kebab, manifest::DocumentManifest, walk::Walker, Author, Date};

const DEFAULT_LOCALE: &str = "en_US";

/// Enumerates the types of documents that can be generated.
///
/// The type dictates the template that will be used to generate the document.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
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

/// In-memory representation of a document.
///
/// # Examples
///
/// ```
/// use djoc::{Document, DocumentType};
///
/// let mut document = Document::default();
/// document
///     .title("My Document")
///     .document_type(DocumentType::Report)
///     .author("John Doe".into())
///     .author("Jane Doe".into())
///     .text("This is the first paragraph.");
///
/// assert_eq!(document.title, "My Document");
/// assert_eq!(document.document_type, DocumentType::Report);
/// assert_eq!(document.authors.len(), 2);
/// ```
pub struct Document {
    pub title: String,
    pub authors: Vec<Author>,
    pub date: Date,
    pub locale: String,
    pub document_type: DocumentType,
    pub(crate) texts: Vec<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "Document".into(),
            locale: DEFAULT_LOCALE.into(),
            texts: Default::default(),
            authors: Default::default(),
            date: Default::default(),
            document_type: Default::default(),
        }
    }
}

impl Document {
    /// Sets the document title.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = title.into();
        self
    }

    /// Adds a text to the document.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.texts.push(text.into());
        self
    }

    /// Adds multiple texts to the document.
    pub fn texts<S: Into<String>>(&mut self, texts: impl IntoIterator<Item = S>) -> &mut Self {
        texts.into_iter().for_each(|text| {
            self.text(text);
        });
        self
    }

    /// Adds an author to the document.
    pub fn author(&mut self, author: Author) -> &mut Self {
        self.authors.push(author);
        self
    }

    /// Adds multiple authors to the document.
    pub fn authors(&mut self, authors: impl IntoIterator<Item = Author>) -> &mut Self {
        self.authors.extend(authors);
        self
    }

    /// Sets the document type.
    pub fn document_type(&mut self, document_type: DocumentType) -> &mut Self {
        self.document_type = document_type;
        self
    }

    /// Sets the date of the document.
    pub fn date(&mut self, date: Date) -> &mut Self {
        self.date = date;
        self
    }

    /// Sets the locale for the document.
    ///
    /// All locales present in the [`pure-rust-locales`] crate are supported. In
    /// general, most [BCP 47] language tags are supported.
    ///
    /// [`pure-rust-locales`]: https://docs.rs/pure-rust-locales
    /// [BCP 47]: https://tools.ietf.org/html/bcp47
    pub fn locale(&mut self, locale: impl Into<String>) -> &mut Self {
        self.locale = locale.into();
        self
    }

    /// Creates a new document from a path. If the path points to a Djot file,
    /// the document will be loaded from the file. If the path points to a
    /// directory, the directory will be recursively walked and all Djot files
    /// will be loaded.
    ///
    /// # Errors
    ///
    /// This function will return an error if the path does not exist or if any
    /// of the files cannot be read.
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let mut texts = Vec::new();
        extend_texts(&path, &mut texts)?;

        Ok(Self {
            texts,
            ..Default::default()
        })
    }

    /// Produces a filename for naming the output file(s).
    pub fn filename(&self) -> String {
        kebab(&self.title)
    }
}

impl<S: Into<String>> From<S> for Document {
    fn from(content: S) -> Self {
        Self {
            texts: vec![content.into()],
            ..Default::default()
        }
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
            date: def.date.map(|d| d.into()).unwrap_or_default(),
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
