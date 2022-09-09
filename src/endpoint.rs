use std::borrow::Cow;

use http::Method;

use crate::Parameters;

/// A trait for endpoint objects.
pub trait Endpoint {
    fn method(&self) -> Method;

    fn path(&self) -> Cow<'static, str>;

    fn parameters(&self) -> Parameters {
        Parameters::new()
    }

    fn body(&self) -> Option<Vec<u8>> {
        None
    }
}
