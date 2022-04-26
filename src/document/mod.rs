mod builder;
mod chapter;

pub use builder::*;
pub use chapter::*;

use crate::{
    bib,
    config::Config,
    error::Error,
    pandoc::{html_template, latex_template, lua, Pandoc, PandocFormat, PandocOption},
};
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

pub struct Document {
    pub chapters: Vec<Chapter>,
    pub config: Config,
    pub root: Option<PathBuf>,
}

impl Document {
    pub fn from(content: impl Into<String>) -> Self {
        Document {
            chapters: vec![Chapter::new(content)],
            config: Config::default(),
            root: None,
        }
    }

    fn content(&self) -> Option<String> {
        let mut content = String::new();
        for ch in self.chapters.iter() {
            content.push_str(&ch.content);
            content.push_str("\n\n");
        }
        if content.trim().is_empty() {
            None
        } else {
            Some(content)
        }
    }

    fn setup_pandoc(&self) -> Result<Pandoc> {
        let mut pandoc = Pandoc::new();

        for filter in lua::get_filters()?.iter() {
            pandoc.push_opt(PandocOption::LuaFilter(filter.to_owned()))
        }

        for bib_file in bib::get_bib_files(self.root.as_ref()).iter() {
            pandoc.push_opt(PandocOption::Bibliography(bib_file.to_owned()));
        }

        for author in self.config.authors.iter() {
            pandoc.push_opt(PandocOption::Author(author.to_owned()));
        }

        if self.config.style.number_sections {
            pandoc.push_opt(PandocOption::NumberSections);
        }

        if let Some(class) = &self.config.style.document_class {
            pandoc.push_opt(PandocOption::DocumentClass(class.to_owned()));
        }

        pandoc.push_opt(PandocOption::Csl(bib::get_csl(&self.config.bib.csl)?));
        pandoc.push_opt(PandocOption::Title(self.config.title.to_owned()));
        pandoc.push_opt(PandocOption::Date(self.config.date()));
        pandoc.push_opt(PandocOption::Citeproc);
        pandoc.push_opt(PandocOption::LinkCitations);
        pandoc.push_opt(PandocOption::Standalone);
        pandoc.push_opt(PandocOption::From(PandocFormat::Markdown));

        Ok(pandoc)
    }

    pub fn latex(&self) -> Result<String> {
        Ok(String::from_utf8(self.latex_bytes()?)?)
    }

    pub fn latex_bytes(&self) -> Result<Vec<u8>> {
        if let Some(content) = self.content() {
            let mut pandoc = self.setup_pandoc()?;

            if let Some(header) = self.config.latex_header() {
                pandoc.include_in_header(&header)?;
            }

            if let Some(title_script) = &self.config.latex.title_script {
                pandoc.push_opt(PandocOption::TitleScript(title_script.to_owned()))
            }

            pandoc.push_opt(PandocOption::Template(latex_template()?));
            pandoc.push_opt(PandocOption::To(PandocFormat::Latex));
            pandoc.run(content.as_bytes()).context("Pandoc errored.")
        } else {
            Err(anyhow!("The document has no content.")).context("Cannot convert to LaTeX.")
        }
    }

    pub fn html_bytes(&self) -> Result<Vec<u8>> {
        if let Some(content) = self.content() {
            let mut pandoc = self.setup_pandoc()?;
            pandoc.push_opt(PandocOption::Katex);
            pandoc.push_opt(PandocOption::Template(html_template()?));
            pandoc.push_opt(PandocOption::To(PandocFormat::Html));
            pandoc.run(content.as_bytes()).context("Pandoc errored.")
        } else {
            Err(anyhow!("The document has no content.")).context("Cannot convert to LaTeX.")
        }
    }

    pub fn pdf_bytes(&self) -> Result<Vec<u8>> {
        let filename = &self.config.filename();

        let mut status = crate::log::MdocTectonicStatusBackend {
            tidy_logs: self.config.build.tidy_logs,
        };

        let config = tectonic::config::PersistentConfig::open(false)
            .map_err(Error::Tectonic)
            .context("Failed to open the default configuration file.")?;

        let only_cached = false;
        let bundle = config
            .default_bundle(only_cached, &mut status)
            .map_err(Error::Tectonic)
            .context("Failed to load the default resource bundle.")?;

        let format_cache_path = config
            .format_cache_path()
            .map_err(Error::Tectonic)
            .context("Failed to set up the format cache.")?;

        let mut files = {
            let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
            sb.bundle(bundle)
                .primary_input_buffer(&self.latex_bytes()?)
                .tex_input_name(&format!("{}.tex", filename))
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .build_date(std::time::SystemTime::now())
                .do_not_write_output_files();

            let mut sess = sb
                .create(&mut status)
                .map_err(Error::Tectonic)
                .context("Failed to initialize the LaTeX processing session.")?;

            sess.run(&mut status)
                .map_err(Error::Tectonic)
                .context("The LaTeX engine failed.")?;

            sess.into_file_data()
        };

        match files.remove(&format!("{}.pdf", filename)) {
            Some(file) => Ok(file.data),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "LaTeX didn't report failure, but no PDF was created.",
            )
            .into()),
        }
    }
}
