use std::{
    io::{self, ErrorKind},
    ops::Deref,
};

use thiserror::Error;
use zerocopy::{CastError, KnownLayout, SizeError, TryCastError, TryFromBytes};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid data: {0}")]
    InvalidData(std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(std::io::Error),
}

impl Error {
    pub fn invalid_size(expect: usize, found: usize) -> Self {
        let source = io::Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid size: expected {expect}, found {found}."),
        );
        Self::InvalidInput(source)
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::InvalidData(e) => e,
            Error::InvalidInput(e) => e,
        }
    }
}

impl<Src, Dst> From<SizeError<Src, Dst>> for Error
where
    Src: Deref,
    Dst: KnownLayout,
{
    fn from(error: SizeError<Src, Dst>) -> Self {
        let source = io::Error::new(ErrorKind::InvalidInput, error.to_string());
        Self::InvalidInput(source)
    }
}

impl<Src, Dst> From<TryCastError<Src, Dst>> for Error
where
    Src: Deref,
    Dst: TryFromBytes + KnownLayout + ?Sized,
{
    fn from(error: TryCastError<Src, Dst>) -> Self {
        let (kind, msg) = match error {
            TryCastError::Size(e) => (ErrorKind::InvalidInput, e.to_string()),
            TryCastError::Validity(e) => (ErrorKind::InvalidData, e.to_string()),
            TryCastError::Alignment(e) => (ErrorKind::InvalidData, e.to_string()),
        };
        let source = io::Error::new(kind, msg);
        Self::InvalidData(source)
    }
}

impl<Src, Dst> From<CastError<Src, Dst>> for Error
where
    Src: Deref,
    Dst: TryFromBytes + KnownLayout + ?Sized,
{
    fn from(error: CastError<Src, Dst>) -> Self {
        let (kind, msg) = match error {
            CastError::Size(e) => (ErrorKind::InvalidInput, e.to_string()),
            CastError::Alignment(e) => (ErrorKind::InvalidData, e.to_string()),
        };
        let source = io::Error::new(kind, msg);
        Self::InvalidData(source)
    }
}
