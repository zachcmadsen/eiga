use serde::de::DeserializeOwned;

use crate::endpoint::Endpoint;
use crate::error::Error;

/// A trait for objects that send requests.
///
/// Clients are defined by one method, `send`. The `send` method sends a
/// request to some endpoint and returns the response.
pub trait Client {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned;
}
