use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::Endpoint;

/// The languages configuration endpoint.
#[derive(Builder, Debug)]
pub struct Languages {}

impl Endpoint for Languages {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "configuration/languages".into()
    }
}
