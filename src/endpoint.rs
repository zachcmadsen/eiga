use std::borrow::Cow;

use http::Method;

use crate::QueryParameters;

/// A trait for endpoint objects.
pub trait Endpoint {
    fn method(&self) -> Method;

    fn path(&self) -> Cow<'static, str>;

    fn parameters(&self) -> QueryParameters {
        QueryParameters::new()
    }

    fn body(&self) -> Option<Vec<u8>> {
        None
    }
}
