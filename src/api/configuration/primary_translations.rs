use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::Endpoint;

/// The primary translations configuration endpoint.
#[derive(Builder, Debug)]
pub struct PrimaryTranslations {}

impl Endpoint for PrimaryTranslations {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "configuration/primary_translations".into()
    }
}
