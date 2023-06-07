use std::path::PathBuf;

use crate::manifest::BuilderManifest;

const DEFAULT_LOCALE: &str = "en_US";

// NOTE: Allow dead code to avoid compiler warnings when all features are
// disabled
#[allow(dead_code)]
/// Struct responsible for building a document.
///
/// It is usually constructed by using the [`Builder::default`] method.
#[derive(Clone)]
pub struct Builder {
    pub(crate) number_sections: bool,
    pub(crate) build_dir: Option<PathBuf>,
    pub(crate) locale: String,
    pub(crate) add_title: bool,
    pub(crate) standalone: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            number_sections: false,
            build_dir: None,
            locale: DEFAULT_LOCALE.to_string(),
            add_title: false,
            standalone: true,
        }
    }
}

impl Builder {
    pub(crate) fn from_manifest(manifest: &BuilderManifest) -> Self {
        Self {
            number_sections: manifest.number_sections.unwrap_or(false),
            locale: manifest.locale.clone().unwrap_or(DEFAULT_LOCALE.into()),
            build_dir: manifest.build_dir.clone(),
            add_title: manifest.add_title.unwrap_or(false),
            standalone: true,
        }
    }

    /// Set whether to number sections in the built document.
    ///
    /// # Examples
    ///
    /// ```
    /// use djoc::Builder;
    ///
    /// let builder = Builder::default().number_sections(true);
    /// ```
    pub fn number_sections(&mut self, number_sections: bool) -> &mut Self {
        self.number_sections = number_sections;
        self
    }

    /// Set the directory where any build artifacts will be placed (e.g.
    /// auxiliary files and/or logs.)
    ///
    /// If not set, only the output will be written to disk.
    pub fn build_dir(&mut self, build_dir: PathBuf) -> &mut Self {
        self.build_dir = Some(build_dir);
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

    pub fn standalone(&mut self, standalone: bool) -> &mut Self {
        self.standalone = standalone;
        self
    }

    pub fn add_title(&mut self, add_title: bool) -> &mut Self {
        self.add_title = add_title;
        self
    }
}
