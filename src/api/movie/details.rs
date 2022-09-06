use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::{Endpoint, QueryParameters};

/// The movie details endpoint.
#[derive(Builder, Debug)]
pub struct Details<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> Endpoint for Details<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}", self.id).into()
    }

    fn parameters(&self) -> QueryParameters {
        let mut parameters = QueryParameters::with_capacity(1);
        parameters.push("language", self.language);

        parameters
    }
}
