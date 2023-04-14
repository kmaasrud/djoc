use std::fmt::Display;

/// Represents an author of a document.
#[derive(Clone)]
pub struct Author {
    /// The name of the author.
    pub name: String,
    /// The email of the author.
    pub email: Option<String>,
    /// The affiliation of the author.
    pub affiliation: Option<String>,
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(email) = &self.email {
            write!(f, " <{}>", email)?;
        }
        if let Some(affiliation) = &self.affiliation {
            write!(f, " ({})", affiliation)?;
        }
        Ok(())
    }
}

impl<S: Into<String>> From<S> for Author {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            email: None,
            affiliation: None,
        }
    }
}
