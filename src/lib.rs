#[macro_use]
pub mod log;

pub mod bib;
pub mod config;
pub mod document;
pub mod error;
pub(crate) mod pandoc;
pub mod utils;

#[doc(inline)]
pub use document::DocumentBuilder;
#[doc(inline)]
pub use document::{Chapter, Document};

pub const CONFIG_FILE: &str = "mdoc.toml";
