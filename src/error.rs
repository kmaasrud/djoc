use std::fmt::Display;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Fmt(std::fmt::Error),
    Io(std::io::Error),
    Tectonic(tectonic::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Fmt(e) => e.fmt(f),
            Error::Io(e) => e.fmt(f),
            Error::Tectonic(e) => e.fmt(f),
            Error::TomlDe(e) => e.fmt(f),
            Error::TomlSer(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Fmt(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Tectonic(e) => Some(e),
            Error::TomlDe(e) => Some(e),
            Error::TomlSer(e) => Some(e),
        }
    }
}

// TODO: This is required by anyhow, but I ideally want to remove that dependency
unsafe impl Sync for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Self::Fmt(value)
    }
}

impl From<tectonic::Error> for Error {
    fn from(value: tectonic::Error) -> Self {
        Self::Tectonic(value)
    }
}
