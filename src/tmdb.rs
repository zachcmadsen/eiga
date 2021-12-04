use std::env;

use async_trait::async_trait;
use reqwest::{header, Client as AsyncClient, Url};
use serde::de::DeserializeOwned;

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
    pub(crate) fn new<S>(token: S) -> TmdbBuilder<'a>
    where
        S: Into<String>,
    {
        TmdbBuilder {
            base_url: None,
            token: token.into(),
        }
    }

    pub(crate) fn from_env() -> Result<TmdbBuilder<'a>, Error> {
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

        let mut headers = header::HeaderMap::new();
        let mut auth =
            header::HeaderValue::from_str(&format!("Bearer {}", self.token))?;
        auth.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth);

        // TODO: Should I add a User-Agent header?
        let client =
            AsyncClient::builder().default_headers(headers).build()?;

        Ok(Tmdb { base_url, client })
    }
}

/// An asynchronous client for sending requests to the TMDB API.
///
/// TODO: Add more info about its use.
///
/// # Example
///
/// TODO: Add multiple examples.
pub struct Tmdb {
    base_url: Url,
    client: AsyncClient,
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
    /// This function returns an error if the environment variable isn't set.
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

#[async_trait]
impl Client for Tmdb {
    async fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint + Sync,
        D: DeserializeOwned,
    {
        let url = self.base_url.join(&endpoint.path())?;
        // TODO: Do I need set a content-type header for the body?
        let body = endpoint.body().unwrap_or_else(Vec::new);

        let response = self
            .client
            .request(endpoint.method(), url)
            .query(&endpoint.parameters())
            .body(body)
            .send()
            .await?;

        let status = response.status();
        let body = response.bytes().await?;

        if !status.is_success() {
            return Err(Error::from_tmdb(&body));
        }

        serde_json::from_slice::<D>(&body).map_err(Error::Json)
    }
}
