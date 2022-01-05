pub mod lua;
pub mod opts;

pub use opts::{PandocFormat, PandocOption};

use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Pandoc {
    path: PathBuf,
    opts: Vec<PandocOption>,
}

impl Pandoc {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("pandoc"),
            opts: vec![],
        }
    }

    pub fn push_opt(&mut self, opt: PandocOption) {
        self.opts.push(opt);
    }

    pub fn run(&self, buf: &[u8]) -> Result<Vec<u8>> {
        let args: Vec<String> = self.opts.iter().map(ToString::to_string).collect();

        debug!("Running Pandoc with opts: {:#?}", args);

        let mut cmd = Command::new(&self.path)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = cmd.stdin.as_mut().context("Failed to open stdin.")?;

        stdin.write_all(buf).context("Failed to write to stdin.")?;

        Ok(cmd.wait_with_output()?.stdout)
    }
}
