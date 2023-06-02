use http::{Request, Response};
use hyper::{client::HttpConnector, Body};
use serde::de::DeserializeOwned;
use tower::{
    util::{BoxCloneService, BoxService},
    BoxError, Service, ServiceExt,
};

use crate::{Endpoint, Error, PageIter, Pageable};

pub struct Client2 {
    service: BoxCloneService<Request<Body>, Response<Body>, hyper::Error>,
}

impl Client2 {
    pub fn new() -> Client2 {
        let client = hyper::Client::builder().build(HttpConnector::new());

        Self {
            service: BoxCloneService::new(client),
        }
    }

    pub async fn raw<T>(&self, request: Request<Body>) -> () {
        let resp = self
            .service
            .clone()
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        let bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();

        // serde_json::from_slice(&full).map_err(crate::error::decode)
    }
}

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
