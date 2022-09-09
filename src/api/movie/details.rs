use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::{Endpoint, Parameters};

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

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::new();
        parameters.push("language", self.language);

        parameters
    }
}
