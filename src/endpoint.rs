use std::borrow::Cow;

use http::Method;

use crate::query::QueryPairs;

/// A trait for endpoint objects.
///
/// All of the structs in this crate that represent endpoints, e.g.,
/// `SearchMovie`, implement this trait. If you want to add an endpoint that's
/// not in the crate, all you have to do is implement this trait. Then you can
/// send requests to the endpoint with `Tmdb` (or another type that implements
/// `Client`).
///
/// # Example
///
/// TODO: Add an example that shows how to implement this for a new endpoint.
pub trait Endpoint {
    fn method(&self) -> Method;

    fn path(&self) -> Cow<'static, str>;

    fn parameters(&self) -> QueryPairs {
        QueryPairs::new()
    }

    fn body(&self) -> Option<Vec<u8>> {
        None
    }
}
