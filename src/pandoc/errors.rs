use thiserror::Error;

#[derive(Debug, Error)]
pub enum PandocError {
    #[error("Pandoc IO error:")]
    Io,
    #[error("Pandoc app error:")]
    App,
    #[error("Pandoc template error:")]
    Template,
    #[error("Pandoc option error:")]
    Option,
    #[error("Unknown Pandoc reader.")]
    UnknownReader,
    #[error("Unknown Pandoc writer.")]
    UnknownWriter,
    #[error("Unsupported extension for Pandoc.")]
    UnsupportedExtension,
    #[error("Citeproc error:")]
    Citeproc,
    #[error("Bibliography error:")]
    Bibliography,
    #[error("Epub subdirectory error.")]
    EpubSubdirectory,
    #[error("Pandoc PDF error:")]
    Pdf,
    #[error("Pandoc XML error:")]
    Xml,
    #[error("PDF program not found.")]
    PDFProgramNotFound,
    #[error("Pandoc HTTP error:")]
    Http,
    #[error("This Pandoc error should not happen.")]
    ShouldNeverHappen,
    #[error("Pandoc parse error.")]
    Parse,
    #[error("Pandoc parsec error.")]
    Parsec,
    #[error("PandocMakePDFError.")]
    MakePDF,
    #[error("Pandoc syntax map error:")]
    SyntaxMap,
    #[error("Pandoc filter error:")]
    Filter,
    #[error("Pandoc Lua filter error:")]
    Lua,
    #[error("Pandoc macro loop.")]
    MacroLoop,
    #[error("UTF-8 decoding error in Pandoc.")]
    Utf8Decoding,
    #[error("Unsupported charset.")]
    UnsupportedCharset,
    #[error("Could not find data file:")]
    CouldNotFindDataFile,
    #[error("Resource not found:")]
    ResourceNotFound,
    #[error("Pandoc error:")]
    Other,
}

impl PandocError {
    pub fn from_code(code: i32, msg: &str) -> Self {
        match code {
            1 => Self::Io,
            4 => Self::App,
            5 => Self::Template,
            6 => Self::Option,
            21 => Self::UnknownReader,
            22 => Self::UnknownWriter,
            23 => Self::UnsupportedExtension,
            24 => Self::Citeproc,
            25 => Self::Bibliography,
            31 => Self::EpubSubdirectory,
            43 => Self::Pdf,
            44 => Self::Xml,
            47 => Self::PDFProgramNotFound,
            61 => Self::Http,
            62 => Self::ShouldNeverHappen,
            64 => Self::Parse,
            65 => Self::Parsec,
            66 => Self::MakePDF,
            67 => Self::SyntaxMap,
            83 => Self::Filter,
            84 => Self::Lua,
            91 => Self::MacroLoop,
            92 => Self::Utf8Decoding,
            94 => Self::UnsupportedCharset,
            97 => Self::CouldNotFindDataFile,
            99 => Self::ResourceNotFound,
            _ => Self::Other,
        }
    }
}
