use std::fmt;

/// All possible errors that can occur when building endpoints and sending
/// requests.
#[derive(Debug)]
pub enum Error {
    InvalidHeaderValue(http::header::InvalidHeaderValue),
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    Tmdb(String),
    Url(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidHeaderValue(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Reqwest(ref err) => err.fmt(f),
            Error::Tmdb(ref s) => s.fmt(f),
            Error::Url(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(err: http::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}
