pub mod bib;
mod document;
pub mod error;
pub(crate) mod latex;
pub mod log;
pub mod manifest;
pub mod utils;
pub(crate) mod walk;

pub use document::{Author, Chapter, Document};

pub const MANIFEST_FILE: &str = "djoc.toml";
