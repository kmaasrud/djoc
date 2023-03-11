mod log;

mod author;
pub mod bib;
mod chapter;
mod document;
pub mod error;
pub(crate) mod latex;
pub(crate) mod manifest;
pub mod utils;
pub(crate) mod walk;

pub use author::Author;
pub use chapter::Chapter;
pub use document::Document;

pub const DOC_DEF_FILE: &str = "djoc.toml";
