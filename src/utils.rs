use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// Finds the root by looking for a `djoc.toml` file. If a `djoc.toml` file is
/// not found, the current directory is returned.
pub fn find_root<P: AsRef<Path>>(start_path: P) -> PathBuf {
    let start_path = start_path.as_ref().to_path_buf();
    let mut path: PathBuf = start_path.clone();
    let look_for = Path::new("djoc.toml");
    loop {
        path.push(look_for);
        if path.is_dir() {
            path.pop();
            return path;
        }
        if !(path.pop() && path.pop()) {
            return start_path;
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

/// Returns the djoc data directory
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
