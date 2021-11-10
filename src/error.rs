use std::{error, io, result, str};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetstackError {
    #[error("Failed to convert bytes to utf8: {0}")]
    Utf8Error(#[from] str::Utf8Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Unknown(#[from] Box<dyn error::Error>),
}

pub type Result<T> = result::Result<T, NetstackError>;
