pub mod utils;

use std::process::{Command, Stdio};
use std::io::Write;

pub struct Document {
    content: String,
}

impl Document {
    pub fn from_str(content: impl Into<String>) -> Self {
        Document { content: content.into() }
    }

    pub fn latex(&self) -> String {
        let mut pandoc = Command::new("pandoc")
            .args([
                "-s",
                "--from=markdown",
                "--to=latex",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdin = pandoc.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(&self.content.as_bytes()).expect("Failed to write");
        drop(stdin);

        String::from_utf8_lossy(&pandoc.wait_with_output().expect("Failed to read output").stdout).into()
    }
}
