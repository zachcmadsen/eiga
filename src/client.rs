use serde::de::DeserializeOwned;

use crate::endpoint::Endpoint;
use crate::error::Error;

/// A trait for objects that send requests.
pub trait Client {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned;

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint;
}
