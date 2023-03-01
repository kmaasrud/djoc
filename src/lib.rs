mod log;

mod author;
pub mod bib;
mod chapter;
mod document;
pub mod error;
pub(crate) mod structure;
pub mod utils;
pub(crate) mod walk;

pub use author::Author;
pub use chapter::Chapter;
pub use document::Document;

pub const CONFIG_FILE: &str = "djoc.toml";
