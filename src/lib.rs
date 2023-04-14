//! djoc is a library for generating documents in various formats.
//!
//! To use djoc in your project, add this to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! djoc = "0.1.0"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use djoc::{Builder, Document, DocumentType};
//!
//! let mut document = Document::from("This is some text");
//! document
//!    .title("My Document")
//!    .document_type(DocumentType::Report)
//!    .author("John Doe");
//!
//! let builder = Builder::default();
//! let mut bytes = Vec::new();
//! builder.write_latex(&document, &mut bytes).unwrap();
//!
//! let latex = String::from_utf8(bytes).unwrap();
//!
//! assert!(latex.contains(r"\documentclass{report}"));
//! assert!(latex.contains("My Document"));
//! assert!(latex.contains("John Doe"));
//! assert!(latex.contains("This is some text"));
//! ```

mod author;
mod builder;
mod date;
mod document;
mod utils;

pub(crate) mod latex;
pub(crate) mod walk;

pub mod html;
pub mod manifest;
pub mod pdf;

pub use author::Author;
pub use builder::Builder;
pub use date::Date;
pub use document::{Document, DocumentType};
#[doc(inline)]
pub use manifest::Manifest;
pub(crate) use utils::kebab;
