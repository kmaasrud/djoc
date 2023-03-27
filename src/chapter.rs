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
        Parser::new(&self.content).flat_map(|event| match event {
            Event::Start(
                Container::Div {
                    class: Some("title"),
                },
                attrs,
            ) => vec![
                Event::Start(Container::RawBlock { format: "latex" }, attrs),
                Event::Str(r"\maketitle".into()),
                Event::End(Container::RawBlock { format: "latex" }),
            ],
            _ => vec![event],
        })
    }

    pub fn write_html<W: fmt::Write>(&self, w: W) -> fmt::Result {
        let mut in_math = false;
        let mut opts = katex::Opts::default();
        let parser = self.get_parser().map(|event| match event {
            Event::Start(Container::Math { display }, attrs) => {
                opts.set_display_mode(display);
                in_math = true;
                Event::Start(Container::RawBlock { format: "html" }, attrs)
            }
            Event::End(Container::Math { .. }) => {
                in_math = false;
                Event::End(Container::RawBlock { format: "html" })
            }
            Event::Str(s) if in_math => {
                Event::Str(katex::render_with_opts(&s, &opts).unwrap().into())
            }
            _ => event,
        });
        html::Renderer::default().push(parser, w)
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
