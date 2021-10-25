use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO-error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error from Tectonic: {0}")]
    Tectonic(String)
} 

impl From<tectonic::Error> for Error {
    fn from(e: tectonic::Error) -> Self {
        Error::Tectonic(e.to_string())
    }
}
