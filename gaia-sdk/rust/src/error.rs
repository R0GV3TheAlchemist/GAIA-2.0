use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GaiaError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("capability denied: {0}")]
    Denied(String),
    #[error("not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, GaiaError>;
