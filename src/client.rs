use crate::api::movie::MovieBuilder;
use crate::api::network::NetworkHandler;
use reqwest::{header, Client, Url};
use serde::{de::DeserializeOwned, Serialize};

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

        // TODO: Should I add a User-Agent header?
        let client = Client::builder().default_headers(headers).build().unwrap();

        // TODO: Add a TmdbBuilder if there are enough configurations.
        Tmdb {
            client,
            base_url: reqwest::Url::parse(TMDB_BASE_URL).unwrap(),
        }
    }

    pub async fn get<A, S, D>(&self, path: A, parameters: Option<&S>) -> Result<D, reqwest::Error>
    where
        A: AsRef<str>,
        S: Serialize + ?Sized,
        D: DeserializeOwned,
    {
        let url = self.base_url.join(path.as_ref()).unwrap();

        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        let response = request.send().await?;

        let text = response.text().await?;
        let de = &mut serde_json::Deserializer::from_str(&text);
        Ok(D::deserialize(de).expect("error in the trait"))
    }
}

impl Tmdb {
    pub fn movie(&self, id: u64) -> MovieBuilder {
        MovieBuilder::new(self, id)
    }

    pub fn network(&self, id: u64) -> NetworkHandler {
        NetworkHandler::new(self, id)
    }
}
