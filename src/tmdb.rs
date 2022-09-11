use serde::de::DeserializeOwned;
use serde::Deserialize;
use ureq::{
    Agent,
    Error::{Status, Transport},
    Header, Response,
};
use url::Url;

use crate::{Client, Endpoint, Error, PageIter, Pageable};

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3/";

/// The TMDB error response body.
#[derive(Deserialize)]
struct TmdbError {
    status_message: String,
}

#[derive(Deserialize)]
struct UnprocessableEntityError {
    errors: Vec<String>,
}

/// A builder for `Tmdb`.
#[derive(Debug)]
pub struct TmdbBuilder<'a> {
    token: String,
    base_url: Option<&'a str>,
}

impl<'a> TmdbBuilder<'a> {
    /// Constructs a new `TmdbBuilder` from the given token.
    fn new<S>(token: S) -> TmdbBuilder<'a>
    where
        S: Into<String>,
    {
        TmdbBuilder {
            token: token.into(),
            base_url: None,
        }
    }

    /// Sets the base URL for requests.
    pub fn base_url(&mut self, base_url: &'a str) -> &mut TmdbBuilder<'a> {
        self.base_url = Some(base_url);

        self
    }

    /// Builds a new `Tmdb` based on the current configuration.
    pub fn build(&self) -> Result<Tmdb, Error> {
        let base_url = Url::parse(self.base_url.unwrap_or(TMDB_BASE_URL))?;

        let auth_header =
            Header::new("authorization", &format!("Bearer {}", self.token));

        // TODO: Should I set the User-Agent header?
        Ok(Tmdb {
            base_url,
            auth_header,
            agent: Agent::new(),
        })
    }
}

/// A client for sending requests to the TMDB API.
#[derive(Debug)]
pub struct Tmdb {
    base_url: Url,
    auth_header: Header,
    agent: Agent,
}

impl Tmdb {
    /// Constructs a new `Tmdb` from the given token.
    ///
    /// Use `Tmdb::builder` if you want to configure the base URL for requests.
    pub fn new<S>(token: S) -> Tmdb
    where
        S: Into<String>,
    {
        // TmdbBuilder only fails if the base URL is invalid. The default URL
        // is valid so it's safe to unwrap here.
        TmdbBuilder::new(token).build().unwrap()
    }

    /// Constructs a new `TmdbBuilder` from the given token.
    pub fn builder<'a, S>(token: S) -> TmdbBuilder<'a>
    where
        S: Into<String>,
    {
        TmdbBuilder::new(token)
    }

    /// Calls the given endpoint and returns the response.
    fn call<E>(&self, endpoint: &E) -> Result<Response, Error>
    where
        E: Endpoint,
    {
        let mut url = self.base_url.join(&endpoint.path())?;
        endpoint.parameters().append_to_url(&mut url);

        let request = self
            .agent
            .request_url(endpoint.method().as_str(), &url)
            // TODO: Is it always safe to unwrap here?
            .set(self.auth_header.name(), self.auth_header.value().unwrap());

        let response = if let Some(body) = endpoint.body() {
            request.send_bytes(&body)
        } else {
            request.call()
        };

        match response {
            Ok(response) => Ok(response),
            Err(Status(422, response)) => {
                let error: UnprocessableEntityError = response.into_json()?;

                Err(Error::Tmdb {
                    code: 422,
                    message: error.errors.join(", "),
                })
            }
            Err(Status(code, response)) => {
                let error: TmdbError = response.into_json()?;

                Err(Error::Tmdb {
                    code,
                    message: error.status_message,
                })
            }
            Err(Transport(transport)) => Err(Error::Transport(transport)),
        }
    }
}

impl Client for Tmdb {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned,
    {
        let response = self.call(endpoint)?;

        response.into_json::<D>().map_err(Error::Deserialize)
    }

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint,
    {
        self.call(endpoint)?;

        Ok(())
    }

    fn page<'a, E, D>(&'a self, endpoint: &'a E) -> PageIter<'a, Self, E, D>
    where
        E: Pageable,
        D: DeserializeOwned,
    {
        PageIter::new(self, endpoint)
    }
}
