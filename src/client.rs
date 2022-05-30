use serde::de::DeserializeOwned;

use crate::endpoint::Endpoint;
use crate::error::Error;
use crate::page::Pageable;

/// A trait for objects that send requests.
pub trait Client {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned;

    fn page<'a, E, D>(
        &'a self,
        endpoint: &'a E,
    ) -> Box<dyn Iterator<Item = Result<Vec<D>, Error>> + 'a>
    where
        E: Pageable,
        D: DeserializeOwned + 'a;

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint;
}
