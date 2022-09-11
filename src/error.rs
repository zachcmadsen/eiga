use std::io;

/// The possible errors that can occur when sending requests.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Deserialization error.
    #[error("failed to deserialize the TMDB response: {}", self)]
    Deserialize(#[from] io::Error),
    /// Invalid URL.
    #[error("failed to parse a URL: {}", self)]
    Url(#[from] url::ParseError),
    /// TMDB API error.
    #[error("TMDB responded with an unexpected status: {}", .message)]
    Tmdb {
        /// The status code.
        code: u16,
        /// The error message.
        message: String,
    },
    /// Transport error.
    #[error("failed to make the request or receive an response: {}", self)]
    Transport(#[from] ureq::Transport),
}
