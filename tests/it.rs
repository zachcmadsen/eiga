use httpmock::prelude::*;
use serde::de::DeserializeOwned;

use eiga::api::movie;
use eiga::endpoint::Endpoint;
use eiga::Client;
use eiga::Error;
use eiga::Tmdb;

/// A builder for `TestClient`.
struct TestClientBuilder<'a> {
    method: Option<&'a str>,
    path: Option<&'a str>,
    parameters: Option<&'a [(&'a str, &'a str)]>,
}

impl<'a> TestClientBuilder<'a> {
    fn build(self) -> TestClient<'a> {
        TestClient {
            method: self.method,
            path: self.path,
            parameters: self.parameters,
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
}

struct TestClient<'a> {
    method: Option<&'a str>,
    path: Option<&'a str>,
    parameters: Option<&'a [(&'a str, &'a str)]>,
}

impl<'a> TestClient<'a> {
    fn builder() -> TestClientBuilder<'a> {
        TestClientBuilder {
            method: None,
            path: None,
            parameters: None,
        }
    }
}

impl<'a> Client for TestClient<'a> {
    fn send<E, D>(&self, _: &E) -> Result<D, Error>
    where
        E: Endpoint,
        D: DeserializeOwned,
    {
        unimplemented!()
    }

    fn ignore<E>(&self, endpoint: &E) -> Result<(), Error>
    where
        E: Endpoint,
    {
        let server = MockServer::start();

        let mock = server.mock(|mut when, then| {
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

            then.status(200);
        });

        let tmdb = Tmdb::builder("<token>")
            .base_url(&server.base_url())
            .build()?;
        tmdb.ignore(endpoint)?;

        mock.assert();

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
