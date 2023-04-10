pub mod html;
pub mod pdf;

use crate::manifest::BuilderManifest;

#[derive(Clone, Default)]
pub struct Builder {
    pub number_sections: bool,
}

impl Builder {
    pub fn from_manifest(manifest: &BuilderManifest) -> Self {
        Self {
            number_sections: manifest.number_sections.unwrap_or(false),
        }
    }
}
