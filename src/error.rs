use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Tectonic(#[from] tectonic::Error),

    #[error(transparent)]
    Pandoc(#[from] crate::pandoc::PandocError),
}

unsafe impl Sync for Error {}
