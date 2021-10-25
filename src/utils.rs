use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

/// Ease-of-use function for creating a file and writing bytes to it
pub fn write_file(path: &Path, bytes: &[u8]) {
    // Create file
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    let mut f = File::create(path).unwrap();

    // Write bytes
    f.write_all(bytes).unwrap();
}
