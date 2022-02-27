pub mod errors;
pub mod lua;
pub mod opts;

pub use errors::PandocError;
pub use opts::{PandocFormat, PandocOption};

use crate::{utils, Error, Result};
use std::io::{BufRead, BufReader, ErrorKind, Write};
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
            .spawn()
            .map_err(|e| {
                if let ErrorKind::NotFound = e.kind() {
                    Error::from(PandocError::Missing) // Pandoc not found error
                } else {
                    e.into() // Other IO error
                }
            })?;

        let stdin = cmd.stdin.as_mut().unwrap();

        stdin.write_all(buf)?;

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

            Some(code) => {
                let err = PandocError::from_code(
                    code,
                    String::from_utf8_lossy(&output.stderr)
                        .lines()
                        .filter(|line| !line.starts_with("[WARNING]"))
                        .collect::<Vec<&str>>()
                        .join("\n"),
                );

                Err(err.into())
            }

            None => {
                let err = PandocError::Other(
                    "Pandoc exited unsuccsessfully, but with no exit code..".to_owned(),
                );

                Err(err.into())
            }
        }
    }
}
