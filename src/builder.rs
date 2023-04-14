use crate::manifest::BuilderManifest;

/// Struct responsible for building a document.
///
/// It is usually constructed by using the [`Builder::default`] method.
#[derive(Clone, Default)]
pub struct Builder {
    pub(crate) number_sections: bool,
}

impl Builder {
    pub(crate) fn from_manifest(manifest: &BuilderManifest) -> Self {
        Self {
            number_sections: manifest.number_sections.unwrap_or(false),
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
}
