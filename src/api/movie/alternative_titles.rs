use std::borrow::Cow;

use eiga_builder_derive::Builder;

use crate::endpoint::Endpoint;
use crate::http::Method;
use crate::query::QueryParameters;

/// The alternative movie titles endpoint.
#[derive(Builder)]
pub struct AlternativeTitles<'a> {
    id: u32,
    country: Option<&'a str>,
}

impl<'a> Endpoint for AlternativeTitles<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}/alternative_titles", self.id).into()
    }

    fn parameters(&self) -> QueryParameters {
        let mut parameters = QueryParameters::with_capacity(1);
        parameters.push("country", self.country);

        parameters
    }
}
