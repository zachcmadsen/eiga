use std::fmt;
use std::io;

/// All possible errors that can occur when building endpoints and sending
/// requests.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Ureq(ureq::Error),
    Url(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(ref err) => err.fmt(f),
            Error::Ureq(ref err) => err.fmt(f),
            Error::Url(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Error::Ureq(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}
