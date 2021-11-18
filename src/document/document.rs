use crate::{config::Config, Chapter, Error};
use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

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

    fn content(&self) -> String {
        let mut content = String::new();
        for ch in self.chapters.iter() {
            content.push_str(&ch.content);
            content.push_str("\n\n");
        }
        content
    }

    fn latex_bytes(&self) -> Result<Vec<u8>> {
        let mut pandoc = Command::new("pandoc")
            .args(["-s", "--from=markdown", "--to=latex"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = pandoc.stdin.as_mut().context("Failed to open stdin.")?;

        stdin
            .write_all(&self.content().as_bytes())
            .context("Failed to write.")?;

        Ok(pandoc
            .wait_with_output()
            .expect("Failed to read output.")
            .stdout)
    }

    pub fn build(&self) -> Result<Vec<u8>> {
        let latex_bytes = self.latex_bytes()?;

        let mut status = crate::log::MdocTectonicStatusBackend;

        let config = tectonic::config::PersistentConfig::open(false)
            .map_err(|e| Error::Tectonic(e))
            .context("Failed to open the default configuration file.")?;

        let only_cached = false;
        let bundle = config
            .default_bundle(only_cached, &mut status)
            .map_err(|e| Error::Tectonic(e))
            .context("Failed to load the default resource bundle.")?;

        let format_cache_path = config
            .format_cache_path()
            .map_err(|e| Error::Tectonic(e))
            .context("Failed to set up the format cache.")?;

        let mut files = {
            let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
            sb.bundle(bundle)
                .primary_input_buffer(&latex_bytes)
                .tex_input_name("mdoc.tex")
                .format_name("latex")
                .format_cache_path(format_cache_path)
                .output_format(tectonic::driver::OutputFormat::Pdf)
                .do_not_write_output_files();

            let mut sess = sb
                .create(&mut status)
                .map_err(|e| Error::Tectonic(e))
                .context("Failed to initialize the LaTeX processing session.")?;

            sess.run(&mut status)
                .map_err(|e| Error::Tectonic(e))
                .context("The LaTeX engine failed.")?;

            sess.into_file_data()
        };

        match files.remove("mdoc.pdf") {
            Some(file) => Ok(file.data),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "LaTeX didn't report failure, but no PDF was created.",
            )
            .into()),
        }
    }
}
