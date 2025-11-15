use std::{
    fmt::Display,
    io::{self, ErrorKind},
    ops::Deref,
};

use thiserror::Error;
use zerocopy::{CastError, KnownLayout, SizeError, TryCastError, TryFromBytes};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    source: std::io::Error,
}

impl Error {
    pub fn invalid_size(expect: usize, found: usize) -> Self {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("Invalid size: expected {expect}, found {found}."),
        )
        .into()
    }
    pub fn unknown_type(expect: impl Display, found: impl Display) -> Self {
        io::Error::new(
            ErrorKind::InvalidData,
            format!("unknown_type: expected {expect}, found {found}."),
        )
        .into()
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        let Error { source } = value;
        source
    }
}

impl<Src, Dst> From<SizeError<Src, Dst>> for Error
where
    Src: Deref,
    Dst: KnownLayout,
{
    fn from(error: SizeError<Src, Dst>) -> Self {
        io::Error::new(ErrorKind::InvalidData, error.to_string()).into()
    }
}

impl<Src, Dst> From<TryCastError<Src, Dst>> for Error
where
    Src: Deref,
    Dst: TryFromBytes + KnownLayout + ?Sized,
{
    fn from(error: TryCastError<Src, Dst>) -> Self {
        io::Error::new(ErrorKind::InvalidData, error.to_string()).into()
    }
}

impl<Src, Dst> From<CastError<Src, Dst>> for Error
where
    Src: Deref,
    Dst: TryFromBytes + KnownLayout + ?Sized,
{
    fn from(error: CastError<Src, Dst>) -> Self {
        io::Error::new(ErrorKind::InvalidData, error.to_string()).into()
    }
}
