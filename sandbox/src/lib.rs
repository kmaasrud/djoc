use djoc::{Builder, Document};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Djoc {
    doc: Document,
    builder: Builder,
}

#[wasm_bindgen]
impl Djoc {
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let mut builder = Builder::default();
        builder.standalone(false);

        Self {
            doc: Document::default(),
            builder,
        }
    }

    pub fn update_doc(&mut self, content: &str) {
        self.doc = content.into();
    }

    pub fn render(&self) -> String {
        let mut s = Vec::new();
        self.builder.write_html(&self.doc, &mut s).unwrap();
        unsafe { std::str::from_utf8_unchecked(&s).trim().to_string() }
    }

    pub fn render_latex(&self) -> String {
        let mut s = Vec::new();
        self.builder.write_latex(&self.doc, &mut s).unwrap();
        unsafe { std::str::from_utf8_unchecked(&s).trim().to_string() }
    }

    pub fn set_title(&mut self, title: &str) {
        if !title.is_empty() {
            self.builder.add_title(true);
            self.doc.title(title);
        }
    }

    pub fn set_author(&mut self, author: &str) {
        self.doc.authors([author]);
    }
}
