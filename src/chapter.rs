use jotdown::{html, Parser, Render};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::{latex, manifest::ChapterManifest};

pub struct Chapter {
    pub title: String,
    content: String,
    pub path: Option<PathBuf>,
}

impl Chapter {
    pub fn new(title: impl ToString, content: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            path: None,
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let content = fs::read_to_string(&path)?;

        Ok(Self {
            title: path.file_stem().unwrap().to_string_lossy().into(),
            content,
            path: Some(path),
        })
    }

    pub fn write_html<W: io::Write>(&self, w: W) -> io::Result<()> {
        html::Renderer.write(Parser::new(&self.content), w)
    }

    pub fn write_latex<W: io::Write>(&self, w: W) -> io::Result<()> {
        latex::Renderer::default().write(Parser::new(&self.content), w)
    }
}

impl TryFrom<ChapterManifest> for Chapter {
    type Error = io::Error;

    fn try_from(def: ChapterManifest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: def.title,
            content: fs::read_to_string(&def.path)?,
            path: Some(def.path),
        })
    }
}
