use std::borrow::Cow;

use eiga_builder_derive::Builder;
use http::Method;

use crate::{Country, Endpoint, Pageable, Parameters};

/// The search movies endpoint.
#[derive(Builder, Debug)]
pub struct Movies<'a> {
    query: &'a str,
    language: Option<&'a str>,
    page: Option<u64>,
    include_adult: Option<bool>,
    region: Option<Country>,
    year: Option<u64>,
    primary_release_year: Option<u64>,
}

impl<'a> Endpoint for Movies<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "search/movie".into()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::new();
        parameters.push("query", Some(self.query));
        parameters.push("language", self.language);
        parameters.push("page", self.page);
        parameters.push("include_adult", self.include_adult);
        parameters.push("region", self.region.as_ref());
        parameters.push("year", self.year);
        parameters.push("primary_release_year", self.primary_release_year);
        parameters
    }
}

impl<'a> Pageable for Movies<'a> {
    fn start_page(&self) -> Option<u64> {
        self.page
    }
}
