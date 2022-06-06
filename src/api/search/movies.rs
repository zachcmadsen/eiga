use std::borrow::Cow;

use crate::endpoint::Endpoint;
use crate::http::Method;
use crate::page::Page;
use crate::query::QueryParameters;

/// A builder for `Movies`.
pub struct MoviesBuilder<'a> {
    query: &'a str,
    language: Option<&'a str>,
    page: Option<u64>,
    include_adult: Option<bool>,
    region: Option<&'a str>,
    year: Option<u64>,
    primary_release_year: Option<u64>,
}

impl<'a> MoviesBuilder<'a> {
    fn new(query: &'a str) -> MoviesBuilder<'a> {
        MoviesBuilder {
            query,
            language: None,
            page: None,
            include_adult: None,
            region: None,
            year: None,
            primary_release_year: None,
        }
    }

    /// Builds a new `Movies` based on the current configuration.
    pub fn build(&self) -> Movies<'a> {
        Movies {
            query: self.query,
            language: self.language,
            page: self.page,
            include_adult: self.include_adult,
            region: self.region,
            year: self.year,
            primary_release_year: self.primary_release_year,
        }
    }

    /// Sets the language query string parameter.
    pub fn language(&mut self, language: &'a str) -> &mut MoviesBuilder<'a> {
        self.language = Some(language);

        self
    }

    /// Sets the page query string parameter.
    pub fn page(&mut self, page: u64) -> &mut MoviesBuilder<'a> {
        self.page = Some(page);

        self
    }

    /// Sets the include_adult query string parameter.
    pub fn include_adult(
        &mut self,
        include_adult: bool,
    ) -> &mut MoviesBuilder<'a> {
        self.include_adult = Some(include_adult);

        self
    }

    /// Sets the region query string parameter.
    pub fn region(&mut self, region: &'a str) -> &mut MoviesBuilder<'a> {
        self.region = Some(region);

        self
    }

    /// Sets the year query string parameter.
    pub fn year(&mut self, year: u64) -> &mut MoviesBuilder<'a> {
        self.year = Some(year);

        self
    }

    /// Sets the primary_release_year query string parameter.
    pub fn primary_release_year(
        &mut self,
        primary_release_year: u64,
    ) -> &mut MoviesBuilder<'a> {
        self.primary_release_year = Some(primary_release_year);

        self
    }
}

/// The search movies endpoint.
pub struct Movies<'a> {
    query: &'a str,
    language: Option<&'a str>,
    page: Option<u64>,
    include_adult: Option<bool>,
    region: Option<&'a str>,
    year: Option<u64>,
    primary_release_year: Option<u64>,
}

impl<'a> Movies<'a> {
    /// Constructs a new `DetailsBuilder` from the given movie ID.
    pub fn builder(query: &'a str) -> MoviesBuilder<'a> {
        MoviesBuilder::new(query)
    }
}

impl<'a> Endpoint for Movies<'a> {
    fn method(&self) -> Method {
        Method::Get
    }

    fn path(&self) -> Cow<'static, str> {
        "search/movie".into()
    }

    fn parameters(&self) -> QueryParameters {
        let mut parameters = QueryParameters::with_capacity(7);
        parameters.push("query", Some(self.query));
        parameters.push("language", self.language);
        parameters.push("page", self.page);
        parameters.push("include_adult", self.include_adult);
        parameters.push("region", self.region);
        parameters.push("year", self.year);
        parameters.push("primary_release_year", self.primary_release_year);
        parameters
    }
}

impl<'a> Page for Movies<'a> {
    fn page(&self) -> Option<u64> {
        self.page
    }
}
