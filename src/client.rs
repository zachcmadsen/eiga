use serde::de::DeserializeOwned;

use crate::{Endpoint, Error, PageIter, Pageable};

/// A trait for objects that send requests.
///
/// Implementors of `Client`, or clients, are defined by three methods, `send`,
/// `ignore`, and `page`:
/// - `send` sends a request to the given endpoint and returns the deserialized
/// response body.
/// - `ignore` sends a request to the given endpoint without deserializing the
/// response body.
/// - `page` returns an iterator over the results of the given pageable
/// endpoint.
pub trait Client {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned;

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint;

    fn page<'a, E, D>(&'a self, endpoint: &'a E) -> PageIter<'a, Self, E, D>
    where
        E: Pageable,
        D: DeserializeOwned;
}
