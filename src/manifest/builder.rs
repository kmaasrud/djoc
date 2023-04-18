use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuilderManifest {
    #[serde(default, alias = "output")]
    pub outputs: Vec<Output>,
    pub number_sections: Option<bool>,
    pub build_dir: Option<PathBuf>,
    pub locale: Option<String>,
    pub add_title: Option<bool>,
}

impl BuilderManifest {
    pub fn merge(&self, other: Self) -> Self {
        Self {
            outputs: [self.outputs.clone(), other.outputs].concat(),
            number_sections: other.number_sections.or(self.number_sections),
            build_dir: other.build_dir.or_else(|| self.build_dir.clone()),
            locale: other.locale.or_else(|| self.locale.clone()),
            add_title: other.add_title.or(self.add_title),
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
