use serde::de::DeserializeOwned;

use crate::endpoint::Endpoint;
use crate::error::Error;

/// A trait for objects that send requests.
///
/// Implementors of `Client`, or clients, are defined by three methods, `send`,
/// and `ignore`:
/// - `send` sends a request to the given endpoint and returns the deserialized
/// response body.
/// - `ignore` sends a request to the given endpoint without deserializing the
/// response body.
///
/// # Example
///
/// ```no_run
/// use eiga::api::search;
/// use eiga::prelude::*;
///
/// fn main() -> Result<(), eiga::Error> {
///     // Create a `Tmdb` client.
///     let tmdb = Tmdb::new("<token>");
///     
///     // Build an endpoint to search for the movie Pale Flower.
///     let search_movies_endpoint =
///         search::Movies::builder("Pale Flower").build();
///     
///     // Send a request to the endpoint using the client.
///     tmdb.ignore(&search_movies_endpoint)?;
///
///     Ok(())
/// }
/// ```
pub trait Client {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned;

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint;
}
