pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Deserialize(reqwest::Error),
    Request(reqwest::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Deserialize(ref err) => err.source(),
            Error::Request(ref err) => err.source(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Deserialize(ref err) => err.fmt(f),
            Error::Request(ref err) => err.fmt(f),
        }
    }
}
