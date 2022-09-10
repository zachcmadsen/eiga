use std::borrow::Cow;

use http::Method;

use crate::Parameters;

/// A trait for endpoint objects.
///
/// # Example
///
/// [`movie::AlternativeTitles`] implements [`Endpoint`]:
///
/// ```
/// // For normal usage of eiga you don't need to pull `Endpoint` into scope.
/// use eiga::{movie, Endpoint};
///
/// let endpoint = movie::AlternativeTitles::new(34048);
///
/// assert_eq!("GET", endpoint.method().as_str());
/// assert_eq!("movie/34048/alternative_titles", endpoint.path());
/// assert_eq!(None, endpoint.body());
/// ```
///
/// [`movie::AlternativeTitles`]: movie/struct.AlternativeTitles.html
pub trait Endpoint {
    /// Returns the HTTP method of this endpoint.
    fn method(&self) -> Method;

    /// Returns the path of this endpoint.
    fn path(&self) -> Cow<'static, str>;

    /// Returns the query string parameters of this endpoint.
    fn parameters(&self) -> Parameters {
        Parameters::new()
    }

    /// Returns the request body of this endpoint.
    fn body(&self) -> Option<Vec<u8>> {
        None
    }
}
