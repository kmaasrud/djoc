use thiserror::Error;

#[derive(Debug, Error)]
pub enum PandocError {
    #[error("Pandoc not found in PATH")]
    Missing,
    #[error("Pandoc IO error: {0}")]
    Io(String),
    #[error("Pandoc app error: {0}")]
    App(String),
    #[error("Pandoc template error: {0}")]
    Template(String),
    #[error("Pandoc option error: {0}")]
    Option(String),
    #[error("Unknown Pandoc reader.")]
    UnknownReader,
    #[error("Unknown Pandoc writer.")]
    UnknownWriter,
    #[error("Unsupported extension for Pandoc.")]
    UnsupportedExtension,
    #[error("Citeproc error: {0}")]
    Citeproc(String),
    #[error("Bibliography error: {0}")]
    Bibliography(String),
    #[error("Epub subdirectory error.")]
    EpubSubdirectory,
    #[error("Pandoc PDF error: {0}")]
    Pdf(String),
    #[error("Pandoc XML error: {0}")]
    Xml(String),
    #[error("PDF program not found.")]
    PDFProgramNotFound,
    #[error("Pandoc HTTP error: {0}")]
    Http(String),
    #[error("This Pandoc error should not happen.")]
    ShouldNeverHappen,
    #[error("Pandoc parse error.")]
    Parse,
    #[error("Pandoc parsec error.")]
    Parsec,
    #[error("PandocMakePDFError.")]
    MakePDF,
    #[error("Pandoc syntax map error: {0}")]
    SyntaxMap(String),
    #[error("Pandoc filter error: {0}")]
    Filter(String),
    #[error("Pandoc Lua filter error: {0}")]
    Lua(String),
    #[error("Pandoc macro loop.")]
    MacroLoop,
    #[error("UTF-8 decoding error in Pandoc.")]
    Utf8Decoding,
    #[error("Unsupported charset.")]
    UnsupportedCharset,
    #[error("Could not find data file: {0}")]
    CouldNotFindDataFile(String),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Pandoc error: {0}")]
    Other(String),
}

impl PandocError {
    pub fn from_code(code: i32, msg: String) -> Self {
        match code {
            1 => Self::Io(msg),
            4 => Self::App(msg),
            5 => Self::Template(msg),
            6 => Self::Option(msg),
            21 => Self::UnknownReader,
            22 => Self::UnknownWriter,
            23 => Self::UnsupportedExtension,
            24 => Self::Citeproc(msg),
            25 => Self::Bibliography(msg),
            31 => Self::EpubSubdirectory,
            43 => Self::Pdf(msg),
            44 => Self::Xml(msg),
            47 => Self::PDFProgramNotFound,
            61 => Self::Http(msg),
            62 => Self::ShouldNeverHappen,
            64 => Self::Parse,
            65 => Self::Parsec,
            66 => Self::MakePDF,
            67 => Self::SyntaxMap(msg),
            83 => Self::Filter(msg),
            84 => Self::Lua(msg),
            91 => Self::MacroLoop,
            92 => Self::Utf8Decoding,
            94 => Self::UnsupportedCharset,
            97 => Self::CouldNotFindDataFile(msg),
            99 => Self::ResourceNotFound(msg),
            _ => Self::Other(msg),
        }
    }
}
