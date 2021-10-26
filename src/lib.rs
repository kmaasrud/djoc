pub mod errors;
pub mod utils;

pub use errors::Error;

use anyhow::{Result, Context};
use std::process::{Command, Stdio};
use std::io::Write;

pub struct Document {
    content: String,
}

impl Document {
    pub fn from_str(content: impl Into<String>) -> Self {
        Document { content: content.into() }
    }

    pub fn latex(&self) -> Result<String> {
        let mut pandoc = Command::new("pandoc")
            .args([
                "-s",
                "--from=markdown",
                "--to=latex",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = pandoc.stdin.as_mut()
            .context("Failed to open stdin")?;

        stdin.write_all(&self.content.as_bytes())
            .context("Failed to write")?;

        drop(stdin);

        Ok(String::from_utf8_lossy(&pandoc.wait_with_output().expect("Failed to read output").stdout).into())
    }
}

pub fn latex_to_pdf<T: AsRef<str>>(latex: T) -> Result<Vec<u8>> {
    let mut status = tectonic::status::NoopStatusBackend::default();

    let config = tectonic::config::PersistentConfig::open(false)
        .map_err(|e| -> Error { e.into() })
        .context("failed to open the default configuration file")?;

    let only_cached = false;
    let bundle = config.default_bundle(only_cached, &mut status)
        .map_err(|e| -> Error { e.into() })
        .context("failed to load the default resource bundle")?;

    let format_cache_path = config.format_cache_path()
        .map_err(|e| -> Error { e.into() })
        .context("failed to set up the format cache")?;

    let mut files = {
        // Looking forward to non-lexical lifetimes!
        let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
        sb.bundle(bundle)
            .primary_input_buffer(latex.as_ref().as_bytes())
            .tex_input_name("texput.tex")
            .format_name("latex")
            .format_cache_path(format_cache_path)
            .keep_logs(false)
            .keep_intermediates(false)
            .print_stdout(true)
            .output_format(tectonic::driver::OutputFormat::Pdf)
            .do_not_write_output_files();

        let mut sess = sb.create(&mut status)
            .map_err(|e| -> Error { e.into() })
            .context("failed to initialize the LaTeX processing session")?;

        sess.run(&mut status)
            .map_err(|e| -> Error { e.into() })
            .context("the LaTeX engine failed")?;

        sess.into_file_data()
    };

    match files.remove("texput.pdf") {
        Some(file) => Ok(file.data),
        None => Err(Error::Tectonic(
            "LaTeX didn't report failure, but no PDF was created (??)".into()
        ).into()),
    }
}
