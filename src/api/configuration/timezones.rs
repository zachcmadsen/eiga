use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::Endpoint;

/// The timezones configuration endpoint.
#[derive(Builder, Debug)]
pub struct Timezones {}

impl Endpoint for Timezones {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "configuration/timezones".into()
    }
}
