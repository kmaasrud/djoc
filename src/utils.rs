use std::path::PathBuf;

#[allow(dead_code)]
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
