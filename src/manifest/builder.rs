use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuilderManifest {
    #[serde(default, alias = "output")]
    pub outputs: Vec<Output>,
    pub number_sections: Option<bool>,
}

impl BuilderManifest {
    pub fn merge(self, other: &Self) -> Self {
        Self {
            outputs: [self.outputs, other.outputs.clone()].concat(),
            number_sections: other.number_sections.or(self.number_sections),
        }
    }
}

#[derive(Clone)]
pub struct Output {
    pub name: Option<String>,
    pub format: OutputFormat,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    Pdf,
    Html,
    #[serde(alias = "tex")]
    Latex,
}

impl AsRef<str> for OutputFormat {
    fn as_ref(&self) -> &str {
        match self {
            OutputFormat::Pdf => "pdf",
            OutputFormat::Html => "html",
            OutputFormat::Latex => "tex",
        }
    }
}
