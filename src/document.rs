use std::{fs, io, path::Path};

use serde::Deserialize;

use crate::{kebab, manifest::DocumentManifest, walk::Walker, Author, Date};

const DEFAULT_LOCALE: &str = "en_US";

/// Enumerates the types of documents that can be generated.
///
/// The type dictates the template that will be used to generate the document.
#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
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

impl From<&str> for DocumentType {
    fn from(s: &str) -> Self {
        match s {
            "article" => DocumentType::Article,
            "report" => DocumentType::Report,
            "book" => DocumentType::Book,
            _ => DocumentType::default(),
        }
    }
}

/// In-memory representation of a document.
///
/// # Examples
///
/// ```
/// use djoc::{Author, Document, DocumentType};
///
/// let mut document = Document::default();
/// document
///     .title("My Document")
///     .document_type("report")
///     .author("John Doe")
///     .authors(["Jane Doe", "John Smith"])
///     .text("This is the first paragraph.");
///
/// assert_eq!(document.title, "My Document");
/// assert_eq!(document.document_type, DocumentType::Report);
/// assert_eq!(document.authors.len(), 3);
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
    pub(crate) fn from_manifest(manifest: &DocumentManifest) -> io::Result<Self> {
        let mut texts = Vec::new();
        for path in &manifest.texts {
            if path.is_dir() {
                extend_texts(path, &mut texts)?;
            } else {
                texts.push(fs::read_to_string(path)?);
            }
        }

        Ok(Self {
            texts,
            date: manifest.date.map(|d| d.into()).unwrap_or_default(),
            title: manifest.title.to_owned(),
            authors: manifest
                .authors
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            locale: manifest.locale.clone().unwrap_or(DEFAULT_LOCALE.into()),
            document_type: manifest.document_type,
        })
    }

    /// Sets the document title.
    pub fn title<T: Into<String>>(&mut self, title: T) -> &mut Self {
        self.title = title.into();
        self
    }

    /// Adds a text to the document.
    pub fn text<T: Into<String>>(&mut self, text: T) -> &mut Self {
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
    pub fn author<A: Into<Author>>(&mut self, author: A) -> &mut Self {
        self.authors.push(author.into());
        self
    }

    /// Adds multiple authors to the document.
    pub fn authors<A: Into<Author>>(&mut self, authors: impl IntoIterator<Item = A>) -> &mut Self {
        authors.into_iter().for_each(|author| {
            self.author(author);
        });
        self
    }

    /// Sets the document type.
    pub fn document_type<D: Into<DocumentType>>(&mut self, document_type: D) -> &mut Self {
        self.document_type = document_type.into();
        self
    }

    /// Sets the date of the document.
    pub fn date<D: Into<Date>>(&mut self, date: D) -> &mut Self {
        self.date = date.into();
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

impl<S: Into<String>> FromIterator<S> for Document {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        Self {
            texts: iter.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

fn extend_texts(path: impl AsRef<Path>, texts: &mut Vec<String>) -> io::Result<()> {
    Walker::new(path)?
        .filter_extensions(&["dj"])
        .try_for_each(|path| {
            texts.push(fs::read_to_string(path)?);
            Ok(())
        })
}
