use std::env;

use serde::de::DeserializeOwned;
use ureq::{Agent, Header};
use url::Url;

use crate::client::Client;
use crate::endpoint::Endpoint;
use crate::error::Error;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3/";

/// A builder for `Tmdb`.
pub struct TmdbBuilder<'a> {
    token: String,
    base_url: Option<&'a str>,
}

impl<'a> TmdbBuilder<'a> {
    fn new<S>(token: S) -> TmdbBuilder<'a>
    where
        S: Into<String>,
    {
        TmdbBuilder {
            token: token.into(),
            base_url: None,
        }
    }

    fn from_env() -> Result<TmdbBuilder<'a>, Error> {
        let token = env::var("TMDB_TOKEN").unwrap();
        Ok(TmdbBuilder::new(token))
    }

    /// Sets the base URL that's used for all requests.
    pub fn base_url(&mut self, base_url: &'a str) -> &mut TmdbBuilder<'a> {
        self.base_url = Some(base_url);
        self
    }

    /// Builds a new `Tmdb` based on the current configuration.
    pub fn build(&self) -> Result<Tmdb, Error> {
        let base_url = Url::parse(self.base_url.unwrap_or(TMDB_BASE_URL))?;

        let auth_header =
            Header::new("authorization", &format!("Bearer {}", self.token));

        // TODO: Should I set a User-Agent header?
        Ok(Tmdb {
            base_url,
            auth_header,
            agent: Agent::new(),
        })
    }
}

/// A client for sending requests to the TMDB API.
pub struct Tmdb {
    base_url: Url,
    auth_header: Header,
    agent: Agent,
}

impl Tmdb {
    /// Constructs a new `Tmdb` from the given token.
    ///
    /// Use `Tmdb::builder` if you want to configure the base URL for
    /// requests.
    ///
    /// # Errors
    ///
    /// This function returns an error if the token is invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use eiga::Tmdb;
    ///
    /// let token = "<token>";
    /// let tmdb = Tmdb::new(token)?;
    /// # Ok::<(), eiga::error::Error>(())
    /// ```
    pub fn new<S>(token: S) -> Result<Tmdb, Error>
    where
        S: Into<String>,
    {
        TmdbBuilder::new(token).build()
    }

    /// Fetches a token from the `TMDB_TOKEN` environment variable and
    /// constructs a new `Tmdb` with it.
    ///
    /// Use `Tmdb::builder` if you want to configure the base URL for
    /// requests.
    ///
    /// # Errors
    ///
    /// This function returns an error if the environment variable isn't set
    /// or if the token is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use eiga::Tmdb;
    ///
    /// let tmdb = Tmdb::from_env()?;
    /// # Ok::<(), eiga::error::Error>(())
    /// ```
    pub fn from_env() -> Result<Tmdb, Error> {
        TmdbBuilder::from_env()?.build()
    }

    /// Constructs a new `TmdbBuilder` from the given token.
    pub fn builder<'a, S>(token: S) -> TmdbBuilder<'a>
    where
        S: Into<String>,
    {
        TmdbBuilder::new(token)
    }
}

impl Client for Tmdb {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned,
    {
        let url = self.base_url.join(&endpoint.path())?;

        let mut request = self
            .agent
            .request_url(endpoint.method().name(), &url)
            .set(self.auth_header.name(), self.auth_header.value().unwrap());

        for (parameter, value) in endpoint.parameters() {
            request = request.query(parameter, value);
        }

        let response = if let Some(body) = endpoint.body() {
            request.send_bytes(&body)
        } else {
            request.call()
        };

        // TODO: Improve error handling. We should handle different error codes
        // and deserialize the TMDB response.
        response?.into_json::<D>().map_err(Error::Io)
    }
}
