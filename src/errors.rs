use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO-error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Tectonic(#[from] tectonic::Error),
}

unsafe impl Sync for Error {}
