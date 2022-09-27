use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::{Country, Endpoint, Parameters};

/// The alternative movie titles endpoint.
#[derive(Builder, Debug)]
pub struct AlternativeTitles {
    id: u32,
    country: Option<Country>,
}

impl Endpoint for AlternativeTitles {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}/alternative_titles", self.id).into()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::new();
        parameters.push("country", self.country.as_ref());

        parameters
    }
}
