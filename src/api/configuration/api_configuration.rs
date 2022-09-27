use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::Endpoint;

/// The API configuration endpoint.
#[derive(Builder, Debug)]
pub struct ApiConfiguration {}

impl Endpoint for ApiConfiguration {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "configuration".into()
    }
}
