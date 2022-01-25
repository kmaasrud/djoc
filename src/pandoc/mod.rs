pub mod errors;
pub mod lua;
pub mod opts;

pub use opts::{PandocFormat, PandocOption};
pub use errors::PandocError;

use anyhow::{bail, Context, Result};
use crate::utils;
use std::io::{Write, BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Pandoc {
    path: PathBuf,
    opts: Vec<PandocOption>,
    header_inclusion: Option<PathBuf>,
}

impl Pandoc {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("pandoc"),
            opts: vec![],
            header_inclusion: None,
        }
    }

    pub fn push_opt(&mut self, opt: PandocOption) {
        self.opts.push(opt);
    }

    pub(crate) fn include_in_header(&mut self, header: &str) -> Result<()> {
        // This is a workaround to get Pandoc to include a string in the header of the output.
        // Pandoc only accepts files from the command line, so I have to write a temporary file and
        // then delete it later.
        //
        // NOTE: This command can only be run once, as running it a second time will overwrite the
        // file.
        let temp_header_path = utils::data_dir().join("header_inclusion");
        self.header_inclusion = Some(temp_header_path.to_owned());
        utils::write_file(&temp_header_path, header.as_bytes())?;
        self.push_opt(PandocOption::IncludeInHeader(temp_header_path));
        Ok(())
    }

    pub fn run(&self, buf: &[u8]) -> Result<Vec<u8>> {
        let args: Vec<String> = self.opts.iter().map(ToString::to_string).collect();

        debug!("Running Pandoc with opts: {:#?}", args);

        let mut cmd = Command::new(&self.path)
            .args(&args)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = cmd.stdin.as_mut().context("Failed to open stdin.")?;

        stdin.write_all(buf).context("Failed to write to stdin.")?;

        let output = cmd.wait_with_output()?;

        if let Some(ref header_file) = self.header_inclusion {
            std::fs::remove_file(header_file)?;
        }

        match output.status.code() {
            Some(0) => {
                for line in BufReader::new(&*output.stderr).lines() {
                    warn!("{}", line?.trim_start_matches("[WARNING] "));
                }

                Ok(output.stdout)
            }

            // TODO: Handle different Pandoc errors (convert to MDoc error type)
            Some(code) => {
                let _err = PandocError::from_code(code, "");

                error!("Pandoc exited with code {}", code);

                Ok(vec![])
            }

            None => {
                bail!("Pandoc exited unsuccsessfully, but with no exit code..")
            }
        }

    }
}
