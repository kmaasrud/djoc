use crate::manifest::AuthorManifest;

pub struct Author {
    pub name: String,
    pub _organization: Option<String>,
}

impl From<AuthorManifest> for Author {
    fn from(def: AuthorManifest) -> Self {
        Author {
            name: def.name,
            _organization: def.organization,
        }
    }
}
