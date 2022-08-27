use std::io;

/// All possible errors that can occur when building endpoints and sending
/// requests.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to deserialize the TMDB response: {}", self)]
    Deserialize(#[from] io::Error),
    #[error("failed to parse a URL: {}", self)]
    InvalidUrl(#[from] url::ParseError),
    #[error("TMDB responded with an unexpected status: {}", .message)]
    Tmdb { code: u16, message: String },
    #[error("failed to make the request or receive an response: {}", self)]
    Transport(#[from] ureq::Transport),
}
