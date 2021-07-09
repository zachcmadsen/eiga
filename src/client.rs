use crate::api::movie::MovieBuilder;
use crate::from_response::FromResponse;
use reqwest::{header, Client, Url};
use serde::Serialize;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3/";

pub struct Tmdb {
    client: Client,
    base_url: Url,
}

impl Tmdb {
    pub fn new(token: &str) -> Tmdb {
        let mut headers = header::HeaderMap::new();

        let mut auth = header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
        auth.set_sensitive(true);

        headers.insert(header::AUTHORIZATION, auth);

        let client = Client::builder().default_headers(headers).build().unwrap();

        // TODO: Add a TmdbBuilder if there are enough configurations.
        Tmdb {
            client,
            base_url: reqwest::Url::parse(TMDB_BASE_URL).unwrap(),
        }
    }

    pub async fn get<A, S, R>(&self, path: A, parameters: Option<&S>) -> Result<R, reqwest::Error>
    where
        A: AsRef<str>,
        S: Serialize + ?Sized,
        R: FromResponse,
    {
        let url = self.base_url.join(path.as_ref()).unwrap();

        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        let response = request.send().await?;

        R::from(response).await
    }
}

impl Tmdb {
    pub fn movie(&self, id: u64) -> MovieBuilder {
        MovieBuilder::new(self, id)
    }
}
