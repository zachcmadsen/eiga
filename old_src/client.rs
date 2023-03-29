use serde::de::DeserializeOwned;

use crate::{Endpoint, Error, PageIter, Pageable};

/// A trait for sending requests to endpoints.
///
/// Implementors of [`Client`] are called clients.
///
/// # Example
///
/// [`Tmdb`] implements [`Client`]:
///
/// ```no_run
/// use std::error::Error;
///
/// use eiga::{search, Client, Tmdb};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let tmdb = Tmdb::new("<token>");
///     let endpoint = search::Movies::new("Tampopo");
///
///     tmdb.ignore(&endpoint)?;
///     
///     Ok(())
/// }
/// ```
///
/// [`Tmdb`]: struct.Tmdb.html
pub trait Client {
    /// Sends a request to the given endpoint and returns the deserialized
    /// response.
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned;

    /// Sends a request to the given endpoint and ignores the response.
    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint;

    /// Returns an iterator over the results of the given pageable endpoint.
    fn page<'a, E, D>(&'a self, endpoint: &'a E) -> PageIter<'a, Self, E, D>
    where
        E: Pageable,
        D: DeserializeOwned;
}
