use std::borrow::Cow;

use http::Method;

use crate::{endpoint::Endpoint, query::QueryPairs};

/// A builder for `SearchMovie`.
pub struct SearchMovieBuilder<'a> {
    query: &'a str,
    language: Option<&'a str>,
    page: Option<u32>,
    include_adult: Option<bool>,
    region: Option<&'a str>,
    year: Option<u32>,
    primary_release_year: Option<u32>,
}

impl<'a> SearchMovieBuilder<'a> {
    pub(crate) fn new(query: &'a str) -> SearchMovieBuilder<'a> {
        SearchMovieBuilder {
            query,
            language: None,
            page: None,
            include_adult: None,
            region: None,
            year: None,
            primary_release_year: None,
        }
    }

    /// Sets the `language` query string parameter.
    pub fn language(
        &mut self,
        language: &'a str,
    ) -> &mut SearchMovieBuilder<'a> {
        self.language = Some(language);
        self
    }

    /// Sets the `page` query string parameter.
    pub fn page(&mut self, page: u32) -> &mut SearchMovieBuilder<'a> {
        self.page = Some(page);
        self
    }

    /// Sets the `include_adult` query string parameter.
    pub fn include_adult(
        &mut self,
        include_adult: bool,
    ) -> &mut SearchMovieBuilder<'a> {
        self.include_adult = Some(include_adult);
        self
    }

    /// Sets the `region` query string parameter.
    pub fn region(&mut self, region: &'a str) -> &mut SearchMovieBuilder<'a> {
        self.region = Some(region);
        self
    }

    /// Sets the `year` query string parameter.
    pub fn year(&mut self, year: u32) -> &mut SearchMovieBuilder<'a> {
        self.year = Some(year);
        self
    }

    /// Sets the `primary_release_year` query string parameter.
    pub fn primary_release_year(
        &mut self,
        primary_release_year: u32,
    ) -> &mut SearchMovieBuilder<'a> {
        self.primary_release_year = Some(primary_release_year);
        self
    }

    /// Builds a new `SearchMovie` based on the current configuration.
    ///
    /// # Example
    /// 
    /// Build an endpoint to search for *Kwaidan* (1966):
    ///
    /// ```
    /// use eiga::api::search::SearchMovie;
    ///
    /// let search_movie_endpoint = SearchMovie::builder("Kwaidan")
    ///     .language("en")
    ///     .page(2)
    ///     .build();
    /// ```
    pub fn build(&self) -> SearchMovie<'a> {
        SearchMovie {
            query: self.query,
            language: self.language,
            page: self.page,
            include_adult: self.include_adult,
            region: self.region,
            year: self.year,
            primary_release_year: self.primary_release_year,
        }
    }
}

pub struct SearchMovie<'a> {
    query: &'a str,
    language: Option<&'a str>,
    page: Option<u32>,
    include_adult: Option<bool>,
    region: Option<&'a str>,
    year: Option<u32>,
    primary_release_year: Option<u32>,
}

impl<'a> SearchMovie<'a> {
    /// Constructs a new `SearchMovieBuilder` from the given query.
    pub fn builder(query: &'a str) -> SearchMovieBuilder<'a> {
        SearchMovieBuilder::new(query)
    }
}

impl<'a> Endpoint for SearchMovie<'a> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        "search/movie".into()
    }

    fn parameters(&self) -> QueryPairs {
        let mut parameters = QueryPairs::with_capacity(7);
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
