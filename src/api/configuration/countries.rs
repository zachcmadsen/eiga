use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::Endpoint;

/// The countries configuration endpoint.
#[derive(Builder, Debug)]
pub struct Countries {}

impl Endpoint for Countries {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "configuration/countries".into()
    }
}
