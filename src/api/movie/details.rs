use std::borrow::Cow;

use crate::endpoint::Endpoint;
use crate::http::Method;
use crate::query::QueryParameters;

/// A builder for `Details`.
pub struct DetailsBuilder<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> DetailsBuilder<'a> {
    fn new(id: u32) -> DetailsBuilder<'a> {
        DetailsBuilder { id, language: None }
    }

    /// Builds a new `Details` based on the current configuration.
    pub fn build(&self) -> Details<'a> {
        Details {
            id: self.id,
            language: self.language,
        }
    }

    /// Sets the language query string parameter.
    pub fn language(&mut self, language: &'a str) -> &mut DetailsBuilder<'a> {
        self.language = Some(language);

        self
    }
}

/// The movie details endpoint.
pub struct Details<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> Details<'a> {
    /// Constructs a new `DetailsBuilder` from the given movie ID.
    pub fn builder(id: u32) -> DetailsBuilder<'a> {
        DetailsBuilder::new(id)
    }
}

impl<'a> Endpoint for Details<'a> {
    fn method(&self) -> Method {
        Method::Get
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
