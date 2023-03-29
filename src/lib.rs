mod error;

pub use error::Result;

use std::borrow::Cow;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Method, Url,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3/";

pub struct TmdbClient {
    base_url: Url,
    client: Client,
}

impl TmdbClient {
    pub fn new(token: &str) -> TmdbClient {
        let base_url = Url::parse(TMDB_BASE_URL).unwrap();

        let mut auth_header =
            HeaderValue::from_str(&format!("Bearer {token}")).unwrap();
        auth_header.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent("eiga/0.4.0")
            .build()
            .unwrap();

        TmdbClient { base_url, client }
    }

    pub async fn send<D, E>(&self, endpoint: E) -> Result<D>
    where
        D: DeserializeOwned,
        E: Endpoint,
        E::Parameters: Serialize,
    {
        let url = self.base_url.join(&endpoint.path()).unwrap();

        let response = self
            .client
            .request(endpoint.method(), url)
            .query(&endpoint.parameters())
            .send()
            .await
            .map_err(Error::Request)?;

        if !response.status().is_success() {
            println!("{:#?}", response.text().await.unwrap());
            todo!();
        }

        println!("{response:#?}");

        response.json::<D>().await.map_err(Error::Deserialize)
    }
}

#[derive(Clone, Debug)]
pub struct GetMovieDetails<'a> {
    pub movie_id: u64,
    pub parameters: GetMovieDetailsParameters<'a>,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetMovieDetailsParameters<'a> {
    pub language: Option<&'a str>,
    pub append_to_response: Option<&'a str>,
}

pub trait Endpoint {
    type Parameters;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> Cow<'static, str>;

    fn parameters(&self) -> &Self::Parameters;
}

impl<'a> Endpoint for GetMovieDetails<'a> {
    type Parameters = GetMovieDetailsParameters<'a>;

    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    fn path(&self) -> Cow<'static, str> {
        format!("movie/{}", self.movie_id).into()
    }
}
