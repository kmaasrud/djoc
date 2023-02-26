#![feature(vec_push_within_capacity)]

mod log;

pub mod bib;
mod chapter;
mod document;
pub mod error;
pub mod utils;
pub(crate) mod walk;

pub use chapter::Chapter;
#[doc(inline)]
pub use document::Document;

pub const CONFIG_FILE: &str = "djoc.toml";
