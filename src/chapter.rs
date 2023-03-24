use jotdown::{html, Container, Event, Parser, Render};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use crate::{latex, manifest::ChapterManifest};

pub struct Chapter {
    pub title: Option<String>,
    content: String,
    pub path: Option<PathBuf>,
}

impl Chapter {
    pub fn new(content: String) -> Self {
        Self {
            content,
            title: None,
            path: None,
        }
    }

    pub fn with_title(self, title: impl ToString) -> Self {
        Self {
            title: Some(title.to_string()),
            ..self
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = fs::canonicalize(path)?;
        let content = fs::read_to_string(&path)?;

        Ok(Self {
            content,
            title: None,
            path: Some(path),
        })
    }

    fn get_parser(&self) -> impl Iterator<Item = Event> {
        // TODO: This is not ideal, as the mapping allocates quite a lot. When we can clone Events,
        // this will be a lot simpler, as we can just modify the events vector.
        Parser::new(&self.content)
            .map(|event| match event {
                Event::Start(
                    Container::Div {
                        class: Some("title"),
                    },
                    attrs,
                ) => vec![
                    Event::Start(Container::RawBlock { format: "latex" }, attrs.clone()),
                    Event::Str(r"\maketitle".into()),
                    Event::End(Container::RawBlock { format: "latex" }),
                ],
                _ => vec![event],
            })
            .flatten()
    }

    pub fn write_html<W: fmt::Write>(&self, w: W) -> fmt::Result {
        html::Renderer.push(self.get_parser(), w)
    }

    pub fn write_latex<W: fmt::Write>(&self, w: W) -> fmt::Result {
        latex::Renderer::default().push(self.get_parser(), w)
    }
}

impl TryFrom<ChapterManifest> for Chapter {
    type Error = io::Error;

    fn try_from(def: ChapterManifest) -> Result<Self, Self::Error> {
        let content = fs::read_to_string(&def.path)?;
        Ok(Self {
            title: def.title,
            content,
            path: Some(def.path),
        })
    }
}
