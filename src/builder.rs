use std::path::PathBuf;

use crate::manifest::BuilderManifest;

/// Struct responsible for building a document.
///
/// It is usually constructed by using the [`Builder::default`] method.
#[derive(Clone, Default)]
pub struct Builder {
    pub(crate) number_sections: bool,
    pub(crate) build_dir: Option<PathBuf>,
}

impl Builder {
    pub(crate) fn from_manifest(manifest: &BuilderManifest) -> Self {
        Self {
            number_sections: manifest.number_sections.unwrap_or(false),
            build_dir: manifest
                .build_dir
                .clone()
                .or_else(|| Some(PathBuf::from("build"))),
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
}
