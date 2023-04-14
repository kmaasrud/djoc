//! Crate-local module for walking the directory tree.

use std::{
    fs::ReadDir,
    io,
    path::{Path, PathBuf},
};

enum WalkNode {
    Dir(ReadDir),
    File(PathBuf),
}

/// An iterator that walks a directory recursively.
pub struct Walker {
    stack: Vec<WalkNode>,
    nesting: usize,
    max_nesting: usize,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            nesting: 0,
            max_nesting: 10,
        }
    }
}

impl Walker {
    /// Creates a new `Walker` from a path.
    ///
    /// # Errors
    ///
    /// This function will return an error if the path does not exist or if the
    /// process does not have permission to read the path.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut stack = Vec::new();
        let path = path.as_ref();

        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "path does not exist",
            ));
        }

        if path.is_dir() {
            match path.read_dir() {
                Ok(read_dir) => stack.push(WalkNode::Dir(read_dir)),
                Err(e) => return Err(e),
            }
        } else {
            stack.push(WalkNode::File(path.into()));
        }

        Ok(Self {
            stack,
            ..Default::default()
        })
    }

    /// Returns a new iterator that only yields paths with the given extensions.
    pub fn filter_extensions(self, extensions: &'static [&'static str]) -> FilteredWalker {
        FilteredWalker {
            inner: self,
            extensions,
        }
    }
}

impl Iterator for Walker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                Some(WalkNode::Dir(iter)) => {
                    self.nesting += 1;
                    iter.filter_map(|e| e.ok()).for_each(|e| {
                        let path = e.path();
                        if path.is_dir() && self.nesting < self.max_nesting {
                            if let Ok(read_dir) = path.read_dir() {
                                self.stack.push(WalkNode::Dir(read_dir));
                            }
                        } else {
                            self.stack.push(WalkNode::File(path));
                        }
                    });
                }
                Some(WalkNode::File(path)) => return Some(path),
                None => return None,
            }
        }
    }
}

/// An iterator that walks a directory recursively and only yields paths with
/// the given extensions.
pub struct FilteredWalker {
    inner: Walker,
    extensions: &'static [&'static str],
}

impl Iterator for FilteredWalker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find(|path| {
            path.extension()
                .and_then(|ext| ext.to_str().map(|ext| self.extensions.contains(&ext)))
                .unwrap_or_default()
        })
    }
}
