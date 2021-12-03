use std::borrow::Cow;

use http::Method;

use crate::endpoint::Endpoint;
use crate::query::QueryPairs;

/// A builder for `Movie`.
pub struct MovieBuilder<'a> {
    id: usize,
    language: Option<&'a str>,
}

impl<'a> MovieBuilder<'a> {
    pub(crate) fn new(id: usize) -> MovieBuilder<'a> {
        MovieBuilder { id, language: None }
    }

    /// Sets the `language` query string parameter.
    ///
    /// # Example
    /// 
    /// Set the language to English:
    ///
    /// ```
    /// use eiga::api::movie::Movie;
    ///
    /// let movie_endpoint = Movie::builder(42).language("en").build();
    /// ```
    pub fn language(&mut self, language: &'a str) -> &mut MovieBuilder<'a> {
        self.language = Some(language);
        self
    }

    /// Builds a new `Movie` based on the current configuration.
    ///
    /// # Example
    /// 
    /// Build a movie endpoint for *House* (1977):
    ///
    /// ```
    /// use eiga::api::movie::Movie;
    ///
    /// let house_id = 5030;
    /// let movie_endpoint = Movie::builder(house_id).build();
    /// ```
    pub fn build(&self) -> Movie<'a> {
        Movie {
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
/// use eiga::api::movie::Movie;
/// use eiga::client::Client;
/// use eiga::tmdb::Tmdb;
///
/// # async {
/// # #[derive(serde::Deserialize)]
/// # struct MovieDetails;
/// let client = Tmdb::from_env()?;
///
/// let cure_id = 402;
/// let movie_endpoint = Movie::builder(cure_id).build();
///
/// // MovieDetails is a user-defined struct.
/// let movie_details: MovieDetails = client.send(&movie_endpoint).await?;
/// # Ok::<(), eiga::error::Error>(())
/// # };
/// ```
pub struct Movie<'a> {
    id: usize,
    language: Option<&'a str>,
}

impl<'a> Movie<'a> {
    /// Constructs a new `MovieBuilder` from the given movie ID.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::api::movie::Movie;
    ///
    /// let movie_endpoint_builder = Movie::builder(42);
    /// ```
    pub fn builder(id: usize) -> MovieBuilder<'a> {
        MovieBuilder::new(id)
    }
}

impl<'a> Endpoint for Movie<'a> {
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