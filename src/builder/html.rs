use super::Builder;
use crate::Document;
use jotdown::{html, Container, Event, Parser, Render};
use rayon::prelude::*;
use std::io::Write;

pub const MAIN_CSS: &[u8] = include_bytes!("./main.css");
pub const KATEX_CSS: &[u8] = include_bytes!("./katex.css");

impl Builder {
    pub fn write_html<W: Write + Send>(&self, document: &Document, mut w: W) -> std::io::Result<()> {
        writeln!(w, "<!DOCTYPE html>\n<html lang=\"en\">\n<head>")?;
        writeln!(w, "<style>")?;
        w.write_all(MAIN_CSS)?;
        writeln!(w, "</style>")?;
        w.write_all(KATEX_CSS)?;
        writeln!(w, "<body>")?;

        let content: String = document
            .texts
            .par_iter()
            .map(|text| {
                let mut buf = String::new();
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

                html::Renderer::default().push(events, &mut buf).unwrap();
                buf
            })
            .collect();
        w.write_all(content.as_bytes())?;

        writeln!(w, "</body>\n</html>")?;
        Ok(())
    }
}
