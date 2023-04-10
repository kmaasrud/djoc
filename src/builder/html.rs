use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

use jotdown::{html, Container, Event, Parser, Render};
use rayon::prelude::*;

use super::Builder;
use crate::Document;

pub const MAIN_CSS: &[u8] = include_bytes!("./main.css");
pub const KATEX_CSS: &[u8] = include_bytes!("./katex.css");

impl Builder {
    pub fn write_html<W: Write + Send>(
        &self,
        document: &Document,
        mut w: W,
    ) -> Result<(), HtmlError> {
        writeln!(w, "<!DOCTYPE html>\n<html lang=\"en\">\n<head>")?;
        writeln!(w, "<style>")?;
        w.write_all(MAIN_CSS)?;
        writeln!(w, "</style>")?;
        w.write_all(KATEX_CSS)?;
        writeln!(w, "<body>")?;

        document
            .texts
            .par_iter()
            .try_fold_with(String::new(), |mut buf, text| {
                let mut opts = katex::Opts::default();
                let mut in_math = false;
                let events = Parser::new(text).map(|event| match event {
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

                html::Renderer::default()
                    .push(events, &mut buf)
                    .map_err(|e| HtmlError::from(e).document_name(&document.title))?;
                Ok(buf)
            })
            .collect::<Result<Vec<String>, HtmlError>>()?
            .into_iter()
            .try_for_each(|s: String| w.write_all(s.as_bytes()))?;

        writeln!(w, "</body>\n</html>")?;
        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct HtmlError {
    pub document_name: Option<String>,
    pub kind: HtmlErrorKind,
}

impl HtmlError {
    pub fn document_name(self, document_name: &str) -> Self {
        Self {
            document_name: Some(document_name.to_string()),
            ..self
        }
    }
}

impl From<io::Error> for HtmlError {
    fn from(e: io::Error) -> Self {
        Self {
            document_name: None,
            kind: HtmlErrorKind::Io(e),
        }
    }
}

impl From<katex::Error> for HtmlError {
    fn from(e: katex::Error) -> Self {
        Self {
            document_name: None,
            kind: HtmlErrorKind::Katex(e),
        }
    }
}

impl From<fmt::Error> for HtmlError {
    fn from(_: fmt::Error) -> Self {
        Self {
            document_name: None,
            kind: HtmlErrorKind::Render,
        }
    }
}

impl Display for HtmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.document_name {
            write!(f, "{name} - ")?;
        }
        match &self.kind {
            HtmlErrorKind::Io(e) => write!(f, "io error: {e}"),
            HtmlErrorKind::Katex(e) => write!(f, "failed to render math with katex: {e}"),
            HtmlErrorKind::Render => write!(f, "failed to render html"),
        }
    }
}

impl Error for HtmlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            HtmlErrorKind::Io(e) => Some(e),
            HtmlErrorKind::Katex(e) => Some(e),
            HtmlErrorKind::Render => None,
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum HtmlErrorKind {
    Io(io::Error),
    Katex(katex::Error),
    Render,
}
