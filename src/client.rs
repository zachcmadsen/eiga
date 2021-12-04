use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::endpoint::Endpoint;
use crate::error::Error;

/// A trait for objects that send requests asynchronously.
///
/// There's an alternative trait for "blocking" clients: `blocking::Client`.
///
/// Clients are defined by one method, `send`. The `send` method will send a
/// request to some endpoint and return the response.
///
/// `Tmdb` implements `Client` and should work for most use cases. Implement
/// this trait if you want a client with custom behavior. For example, you
/// might want to use a different library to send requests (`Tmdb` uses
/// reqwest). The `Endpoint` trait intentionally uses library agnostic types
/// to work well with any library.
///
/// # Example
///
/// `Tmdb` implements `Client`:
///
/// ```no_run
/// use eiga::api::movie::Movie;
/// use eiga::client::Client;
/// use eiga::tmdb::Tmdb;
///
/// # async {
/// # #[derive(serde::Deserialize)]
/// # struct MovieDetails;
/// let client = Tmdb::from_env()?;
///
/// // Create an endpoint to fetch the details of Harakiri (1962).
/// let harakiri_id = 402;
/// let movie_endpoint = Movie::builder(harakiri_id).build();
///
/// // Send the request! Note that MovieDetails is a user-defined struct.
/// let movie_details: MovieDetails = client.send(&movie_endpoint).await?;
/// # Ok::<(), eiga::error::Error>(())
/// # };
/// ```
#[async_trait]
pub trait Client {
    async fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint + Sync,
        D: DeserializeOwned;
}
