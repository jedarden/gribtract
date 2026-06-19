use std::fmt;

#[derive(Debug)]
pub enum Error {
    TooShort { needed: usize, got: usize },
    BadMagic([u8; 4]),
    UnknownEdition(u8),
    BadSectionNumber { expected: u8, got: u8 },
    Overflow,
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TooShort { needed, got } =>
                write!(f, "buffer too short: need {needed}, got {got}"),
            Error::BadMagic(b) =>
                write!(f, "bad GRIB magic: {:?}", b),
            Error::UnknownEdition(e) =>
                write!(f, "unknown GRIB edition: {e}"),
            Error::BadSectionNumber { expected, got } =>
                write!(f, "unexpected section number: expected {expected}, got {got}"),
            Error::Overflow =>
                write!(f, "integer overflow in section length"),
            Error::NotImplemented =>
                write!(f, "decode not implemented"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
