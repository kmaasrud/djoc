use crate::structure::AuthorDef;

pub struct Author {
    pub _name: String,
    pub _organization: Option<String>,
}

impl From<AuthorDef> for Author {
    fn from(def: AuthorDef) -> Self {
        Author {
            _name: def.name,
            _organization: def.organization,
        }
    }
}
