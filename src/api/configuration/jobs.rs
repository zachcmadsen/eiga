use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::Endpoint;

/// The jobs configuration endpoint.
#[derive(Builder, Debug)]
pub struct Jobs {}

impl Endpoint for Jobs {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "configuration/jobs".into()
    }
}
