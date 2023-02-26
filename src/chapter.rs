use jotdown::{html, Parser};
use std::{io, path::PathBuf};

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

    pub fn write_html<W: io::Write>(&self, w: W) -> io::Result<()> {
        html::write(Parser::new(&self.content), w)?;
        Ok(())
    }

    pub fn write_latex<W: io::Write>(&self, _w: W) -> io::Result<()> {
        Ok(())
    }
}
