use std::io;

/// All possible errors that can occur when building endpoints and sending
/// requests.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "an error occurred while deserializing the response body {}",
        self
    )]
    Io(#[from] io::Error),
    #[error("an error occurred while processing the request {}", self)]
    Ureq(#[from] ureq::Error),
    #[error("invalid URL {}", self)]
    Url(#[from] url::ParseError),
}
