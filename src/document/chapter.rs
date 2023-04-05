use jotdown::{html, Container, Event, Parser, Render};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use crate::{latex, manifest::ChapterManifest};

pub struct Chapter {
    pub title: Option<String>,
    pub path: Option<PathBuf>,
    content: String,
}

impl Chapter {
    pub fn new(content: String) -> Self {
        Self {
            title: None,
            path: None,
            content,
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
        Parser::new(&self.content)
    }

    pub fn write_html<W: fmt::Write>(&self, w: W) -> fmt::Result {
        let mut in_math = false;
        let mut opts = katex::Opts::default();
        let events = self.get_parser().map(|event| match event {
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

        html::Renderer::default().push(events, w)
    }

    pub fn write_latex<W: fmt::Write>(&self, w: W) -> fmt::Result {
        // TODO: This is not ideal, as the mapping allocates quite a lot.
        let events = self.get_parser().flat_map(|event| match event {
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
        });

        latex::Renderer::default().push(events, w)
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