use httpmock::prelude::*;
use httpmock::Mock;
use serde::de::DeserializeOwned;
use ureq::serde_json::{json, Value};

use eiga::api::movie;
use eiga::api::search;
use eiga::endpoint::Endpoint;
use eiga::page::{PageIterator, Pageable};
use eiga::Client;
use eiga::Error;
use eiga::Tmdb;

/// A builder for `TestClient`.
struct TestClientBuilder<'a> {
    method: Option<&'a str>,
    path: Option<&'a str>,
    parameters: Option<&'a [(&'a str, &'a str)]>,
    body: Option<Value>,
}

impl<'a> TestClientBuilder<'a> {
    fn build(self) -> TestClient<'a> {
        let server = MockServer::start();
        let tmdb = Tmdb::builder("<token>")
            .base_url(&server.base_url())
            .build()
            .unwrap();

        TestClient {
            server,
            tmdb,
            method: self.method,
            path: self.path,
            parameters: self.parameters,
            body: self.body,
        }
    }

    fn method(mut self, method: &'a str) -> TestClientBuilder {
        self.method = Some(method);

        self
    }

    fn path(mut self, path: &'a str) -> TestClientBuilder {
        self.path = Some(path);

        self
    }

    fn parameters(
        mut self,
        parameters: &'a [(&'a str, &'a str)],
    ) -> TestClientBuilder {
        self.parameters = Some(parameters);

        self
    }

    fn body(mut self, body: Value) -> TestClientBuilder<'a> {
        self.body = Some(body);

        self
    }
}

struct TestClient<'a> {
    server: MockServer,
    tmdb: Tmdb,
    method: Option<&'a str>,
    path: Option<&'a str>,
    parameters: Option<&'a [(&'a str, &'a str)]>,
    body: Option<Value>,
}

impl<'a> TestClient<'a> {
    fn builder() -> TestClientBuilder<'a> {
        TestClientBuilder {
            method: None,
            path: None,
            parameters: None,
            body: None,
        }
    }

    #[allow(unused_assignments)]
    fn mock(&self) -> Mock {
        self.server.mock(|mut when, mut then| {
            when = when.header("authorization", "Bearer <token>");
            if let Some(method) = self.method {
                when = when.method(method);
            }
            if let Some(path) = self.path {
                when = when.path_contains(path);
            }
            if let Some(parameters) = self.parameters {
                for (parameter, value) in parameters {
                    when = when.query_param(*parameter, *value);
                }
            }

            then = then.status(200);
            if let Some(body) = self.body.clone() {
                then = then.json_body(body);
            }
        })
    }
}

impl<'a> Client for TestClient<'a> {
    fn send<E, D>(&self, endpoint: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned,
    {
        let mut mock = self.mock();

        let result = self.tmdb.send(endpoint);

        mock.assert();
        mock.delete();

        result
    }

    fn page<'b, E, D>(
        &'b self,
        endpoint: &'b E,
    ) -> Box<dyn Iterator<Item = Result<Vec<D>, Error>> + 'b>
    where
        E: Pageable,
        D: DeserializeOwned + 'b,
    {
        Box::new(PageIterator::new(self, endpoint))
    }

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint,
    {
        let mut mock = self.mock();

        self.tmdb.ignore(endpoint)?;

        mock.assert();
        mock.delete();

        Ok(())
    }
}

#[test]
fn movie_details() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500")
        .parameters(&[("language", "en-US")])
        .build();

    let movie_details_endpoint =
        movie::Details::builder(500).language("en-US").build();

    test_client.ignore(&movie_details_endpoint).unwrap();
}

#[test]
fn search_movies() {
    let total_pages = 5;

    let test_client = TestClient::builder()
        .method("GET")
        .path("search/movie")
        .parameters(&[
            ("language", "en-US"),
            ("include_adult", "false"),
            ("region", "US"),
            ("year", "1965"),
            ("primary_release_year", "1965"),
        ])
        .body(json!({"results": [], "total_pages": total_pages}))
        .build();

    let search_movies_endpoint = search::Movies::builder("Samurai Spy")
        .language("en-US")
        .page(1)
        .include_adult(false)
        .region("US")
        .year(1965)
        .primary_release_year(1965)
        .build();

    test_client.ignore(&search_movies_endpoint).unwrap();

    let page_iter = test_client.page::<_, ()>(&search_movies_endpoint);

    assert_eq!(page_iter.count(), total_pages);
}
