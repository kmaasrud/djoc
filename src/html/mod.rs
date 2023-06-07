//! HTML output functionality for djoc.
//!
//! This module only contains the error types for HTML output and provides the
//! [`Builder::write_html`] method.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::{self, Write},
};

use jotdown::{html, Container, Event, Parser, Render};
use rayon::prelude::*;

use super::Builder;
use crate::Document;

const MAIN_CSS: &[u8] = include_bytes!("main.css");
const KATEX_CSS: &[u8] = include_bytes!("katex.css");

impl Builder {
    /// Build the document as HTML and write it to the given writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use djoc::{Builder, Document};
    ///
    /// let builder = Builder::default();
    /// let document = Document::from("Hello, world!".to_string());
    /// let mut bytes = Vec::new();
    /// builder.write_html(&document, &mut bytes).unwrap();
    /// ```
    pub fn write_html<W: Write + Send>(
        &self,
        document: &Document,
        mut w: W,
    ) -> Result<(), HtmlError> {
        let mut inner = || -> Result<(), HtmlError> {
            if self.standalone {
                writeln!(w, "<!DOCTYPE html>\n<html lang=\"en\">\n<head>")?;
                writeln!(w, "<style>")?;
                w.write_all(MAIN_CSS)?;
                writeln!(w, "</style>")?;
                w.write_all(KATEX_CSS)?;

                writeln!(w, "</head>")?;
                writeln!(w, "<body>")?;
            }

            document
                .texts
                .par_iter()
                .try_fold_with(Vec::new(), |mut buf, text| {
                    let mut opts = katex::Opts::builder()
                        .throw_on_error(false)
                        .build()
                        .unwrap();
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

                    html::Renderer::default().write(events, &mut buf)?;
                    Ok(buf)
                })
                .collect::<Result<Vec<Vec<u8>>, HtmlError>>()?
                .into_iter()
                .try_for_each(|s| w.write_all(&s))?;

            if self.standalone {
                writeln!(w, "</body>\n</html>")?;
            }

            Ok(())
        };

        inner().map_err(|e| e.document_name(&document.title))
    }
}

/// An error that can occur when rendering HTML.
#[non_exhaustive]
#[derive(Debug)]
pub struct HtmlError {
    /// The title of the document that caused this error.
    pub document_name: Option<String>,
    /// The kind of error that occurred.
    pub kind: HtmlErrorKind,
}

impl HtmlError {
    /// Set the name of the document that caused this error.
    #[must_use]
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

impl Display for HtmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.document_name {
            write!(f, "{name} - ")?;
        }
        match &self.kind {
            HtmlErrorKind::Io(e) => write!(f, "io error: {e}"),
            HtmlErrorKind::Katex(_) => write!(f, "failed to render math with katex"),
        }
    }
}

impl Error for HtmlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            HtmlErrorKind::Io(source) => Some(source),
            HtmlErrorKind::Katex(source) => Some(source),
        }
    }
}

/// The kind of error that can occur when rendering HTML.
#[non_exhaustive]
#[derive(Debug)]
pub enum HtmlErrorKind {
    /// An error that occurred while writing to the writer.
    Io(io::Error),
    /// An error that occurred while rendering math with [`katex`].
    Katex(katex::Error),
}
