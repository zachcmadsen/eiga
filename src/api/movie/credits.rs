use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::{Endpoint, Parameters};

/// The movie credits endpoint.
#[derive(Builder, Debug)]
pub struct Credits<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> Endpoint for Credits<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}/credits", self.id).into()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::new();
        parameters.push("language", self.language);

        parameters
    }
}
