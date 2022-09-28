use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::{Endpoint, Language, Parameters};

/// The movie credits endpoint.
#[derive(Builder, Debug)]
pub struct Credits {
    id: u64,
    language: Option<Language>,
}

impl Endpoint for Credits {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}/credits", self.id).into()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::new();
        parameters.push("language", self.language.as_ref());

        parameters
    }
}
