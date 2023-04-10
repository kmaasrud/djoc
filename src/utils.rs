use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};

use crate::MANIFEST_FILE;

/// Finds the root of a djoc document by looking for a `djoc.toml` file.
pub fn find_root<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let mut path: PathBuf = path.as_ref().into();
    let look_for = Path::new(MANIFEST_FILE);
    loop {
        path.push(look_for);
        if path.is_file() {
            path.pop();
            return Ok(path);
        }
        if !(path.pop() && path.pop()) {
            return Err(io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to find a \"djoc.toml\" file.",
            ));
        }
    }
}

// Function fetched from https://github.com/rust-lang/mdBook/blob/master/src/cmd/init.rs
/// Obtains author name from git config file by running the `git config`
/// command.
pub fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}

/// Finds the djoc data directory
pub fn data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Unable to get the data directory.")
        .join("djoc")
}

/// Make kebab-cased string
pub fn kebab(s: &str) -> String {
    s.chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() || ch == '-' {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}
