mod builder;
mod chapter;
mod lua;

pub use builder::*;
pub use chapter::*;

use crate::{config::Config, Error};
use anyhow::{Context, Result};
use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command, Stdio};

const PREAMBLE: &[u8] = include_bytes!("preamble.tex");

pub struct Document {
    pub chapters: Vec<Chapter>,
    pub config: Config,
}

impl Document {
    pub fn from(content: impl Into<String>) -> Self {
        Document {
            chapters: vec![Chapter::new(content)],
            config: Config::default(),
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

    fn latex_bytes(&self) -> Result<Vec<u8>> {
        let mut pandoc_args = vec![OsStr::new("--from=markdown"), OsStr::new("--to=latex")];
        let filters = lua::get_filters()?;
        pandoc_args.extend(
            filters
                .iter()
                .map(|l| [OsStr::new("-L"), l.as_os_str()])
                .flatten(),
        );

        let mut pandoc = Command::new("pandoc")
            .args(&pandoc_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = pandoc.stdin.as_mut().context("Failed to open stdin.")?;

        stdin
            .write_all(
                self.content()
                    .ok_or_else(|| anyhow::anyhow!("Document has no content."))?
                    .as_bytes(),
            )
            .context("Failed to write to stdin.")?;

        let mut meta = String::new();
        meta.push_str(&format!(
            "\\title{{{}}}\n\\author{{{}}}\n\\date{{{}}}\n{}",
            self.config.title,
            self.config.date(),
            self.config.latex_authors(),
            self.config.number_sections(),
        ));

        let bytes = [
            PREAMBLE,
            meta.as_bytes(),
            "\n\n\\begin{document}\n\\maketitle\n\n".as_bytes(),
            &pandoc.wait_with_output()?.stdout,
            "\n\n\\end{document}".as_bytes(),
        ]
        .concat();

        Ok(bytes)
    }

    pub fn build(&self) -> Result<Vec<u8>> {
        let latex_bytes = self.latex_bytes()?;
        let filename = &self.config.filename();

        let mut status = crate::log::MdocTectonicStatusBackend;

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
                .primary_input_buffer(&latex_bytes)
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
