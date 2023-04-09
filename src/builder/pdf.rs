use super::Builder;
use crate::{latex, Author, Document};
use jotdown::{Parser, Render};
use rayon::prelude::*;
use sailfish::{runtime::Buffer, TemplateOnce};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

impl Builder {
    pub fn write_pdf<W: Write>(&self, document: &Document, mut w: W) -> std::io::Result<()> {
        let filename = document.filename();
        let build_root = Path::new("build").join(&filename);
        fs::create_dir_all(&build_root)?;

        let mut status = crate::log::LoggingStatusBackend;
        let config = tectonic::config::PersistentConfig::default();
        let bundle = config.default_bundle(false, &mut status)?;

        let format_cache_path = config.format_cache_path()?;

        let mut bytes = Vec::new();
        self.write_latex(document, &mut bytes)?;

        let mut files = {
            let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
            sb.bundle(bundle)
                .primary_input_buffer(&bytes)
                .filesystem_root(&build_root)
                .keep_intermediates(true)
                .keep_logs(true)
                .tex_input_name(&format!("{filename}.tex"))
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .output_dir(&build_root)
                .build_date(SystemTime::now());

            let mut sess = sb.create(&mut status)?;

            sess.run(&mut status)?;

            sess.into_file_data()
        };

        match files.remove(&format!("{filename}.pdf")) {
            Some(file) => w.write_all(&file.data)?,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "LaTeX didn't report failure, but no PDF was created.",
                ))
            }
        }
        Ok(())
    }

    pub fn write_latex<W: Write>(&self, document: &Document, mut w: W) -> std::io::Result<()> {
        let mut title = String::new();
        latex::Renderer::default()
            .push(Parser::new(&document.title), &mut title)
            .unwrap();

        let content = document
            .texts
            .par_iter()
            .map(|text| {
                let mut buf = String::new();
                latex::Renderer::default()
                    .push(Parser::new(text), &mut buf)
                    .unwrap();
                buf
            })
            .collect();

        let mut buf = Buffer::new();
        let tmpl = LatexTemplate {
            title: &title,
            authors: &document.authors,
            date: document.formatted_date(),
            content,
            locale: &document.locale,
            document_type: document.document_type.as_ref().into(),
        };
        tmpl.render_once_to(&mut buf).unwrap();

        w.write_all(buf.into_string().as_bytes())?;

        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "latex.stpl")]
struct LatexTemplate<'a> {
    title: &'a str,
    authors: &'a [Author],
    date: Option<String>,
    content: String,
    locale: &'a str,
    document_type: String,
}
