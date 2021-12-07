#[macro_use]
pub mod log;

pub mod bib;
pub mod config;
pub mod document;
pub mod errors;
pub mod utils;

pub use document::DocumentBuilder;
pub use document::{Chapter, Document};

pub use errors::Error;

pub const CONFIG_FILE: &str = "mdoc.toml";
pub const SRC_DIR: &str = "src";
