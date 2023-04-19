use std::io;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error: {0}")]
    Http(#[from] http::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    WasiStream(#[from] bindings::streams::StreamError),

    #[error("{0}")]
    Other(&'static str),
}

impl Error {
    pub fn other(msg: &'static str) -> Self {
        Self::Other(msg)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(err) => err,
            other => io::Error::new(io::ErrorKind::Other, other),
        }
    }
}
