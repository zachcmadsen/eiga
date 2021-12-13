use std::borrow::Cow;

use http::Method;

use crate::endpoint::Endpoint;
use crate::query::QueryPairs;

/// A builder for `Details`.
///
/// # Example
///
/// Build a movie details endpoint for *House* (1977):
///
/// ```
/// use eiga::api::movie;
///
/// let house_id = 5030;
/// let movie_endpoint = movie::Details::builder(house_id).build();
/// ```
pub struct DetailsBuilder<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> DetailsBuilder<'a> {
    fn new(id: u32) -> DetailsBuilder<'a> {
        DetailsBuilder { id, language: None }
    }

    /// Sets the `language` query string parameter.
    pub fn language(&mut self, language: &'a str) -> &mut DetailsBuilder<'a> {
        self.language = Some(language);
        self
    }

    /// Builds a new `Movie` based on the current configuration.
    pub fn build(&self) -> Details<'a> {
        Details {
            id: self.id,
            language: self.language,
        }
    }
}

/// The endpoint to fetch the details of a movie.
///
/// # Example
///
/// Get details of *Cure* (1997):
///
/// ```no_run
/// use eiga::api::movie;
/// use eiga::client::Client;
/// use eiga::tmdb::Tmdb;
///
/// # async {
/// # #[derive(serde::Deserialize)]
/// # struct MovieDetails;
/// let client = Tmdb::from_env()?;
///
/// let cure_id = 402;
/// let movie_details_endpoint = movie::Details::builder(cure_id).build();
///
/// // MovieDetails is a user-defined struct.
/// let movie_details: MovieDetails =
///     client.send(&movie_details_endpoint).await?;
/// # Ok::<(), eiga::error::Error>(())
/// # };
/// ```
pub struct Details<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> Details<'a> {
    /// Constructs a new `DetailsBuilder` from the given movie ID.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::api::movie;
    ///
    /// let movie_details_endpoint_builder = movie::Details::builder(42);
    /// ```
    pub fn builder(id: u32) -> DetailsBuilder<'a> {
        DetailsBuilder::new(id)
    }
}

impl<'a> Endpoint for Details<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}", self.id).into()
    }

    fn parameters(&self) -> QueryPairs {
        let mut parameters = QueryPairs::new();
        parameters.push("language", self.language);
        parameters
    }
}

/// A builder for `Credits`.
pub struct CreditsBuilder<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> CreditsBuilder<'a> {
    fn new(id: u32) -> CreditsBuilder<'a> {
        CreditsBuilder { id, language: None }
    }

    /// Sets the `language` query string parameter.
    pub fn language(&mut self, language: &'a str) -> &mut CreditsBuilder<'a> {
        self.language = Some(language);
        self
    }

    /// Builds a new `Credits` based on the current configuration.
    pub fn build(&self) -> Credits<'a> {
        Credits {
            id: self.id,
            language: self.language,
        }
    }
}

/// The endpoint to fetch the credits for a movie.
pub struct Credits<'a> {
    id: u32,
    language: Option<&'a str>,
}

impl<'a> Credits<'a> {
    /// Constructs a new `CreditsBuilder` from the given movie ID.
    pub fn builder(id: u32) -> CreditsBuilder<'a> {
        CreditsBuilder::new(id)
    }
}

impl<'a> Endpoint for Credits<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}/credits", self.id).into()
    }

    fn parameters(&self) -> QueryPairs {
        let mut parameters = QueryPairs::new();
        parameters.push("language", self.language);
        parameters
    }
}
