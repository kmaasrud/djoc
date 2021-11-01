use anyhow::{Result, Context};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

pub struct Chapter {
    pub content: String,
    pub path: Option<PathBuf>,
}

impl Chapter {
    pub fn new(content: impl Into<String>) -> Self {
        Self { content: content.into(), path: None } 
    }

    pub fn load(path: impl Into<PathBuf>) -> Result<Self> {
        let path: PathBuf = path.into();
        let file = File::open(&path)
            .with_context(|| format!("Could not open file {:?}", path))?;

        let mut content = String::new();
        BufReader::new(&file).read_to_string(&mut content)
            .with_context(|| format!("Could not read {:?} to string", file))?;

        Ok(Self { path: Some(path), content })
    }
}
