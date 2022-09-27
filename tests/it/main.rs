mod configuration;
mod error;
mod movie;
mod search;

use eiga::{Client, Endpoint, Error, PageIter, Pageable, Tmdb};
use httpmock::prelude::*;
use httpmock::Mock;
use serde::de::DeserializeOwned;
use ureq::serde_json::Value;

struct TestClient<'a> {
    server: MockServer,
    tmdb: Tmdb,
    method: Option<&'a str>,
    path: Option<&'a str>,
    parameters: Option<&'a [(&'a str, &'a str)]>,
    status: Option<u16>,
    response: Option<Value>,
}

impl<'a> TestClient<'a> {
    fn new() -> TestClient<'a> {
        let server = MockServer::start();
        let tmdb = Tmdb::builder("<token>")
            .base_url(&server.base_url())
            .build()
            .unwrap();

        TestClient {
            server,
            tmdb,
            method: None,
            path: None,
            parameters: None,
            status: None,
            response: None,
        }
    }

    fn method(mut self, method: &'a str) -> TestClient<'a> {
        self.method = Some(method);

        self
    }

    fn path(mut self, path: &'a str) -> TestClient<'a> {
        self.path = Some(path);

        self
    }

    fn parameters(
        mut self,
        parameters: &'a [(&'a str, &'a str)],
    ) -> TestClient<'a> {
        self.parameters = Some(parameters);

        self
    }

    fn status(mut self, status: u16) -> TestClient<'a> {
        self.status = Some(status);

        self
    }

    fn response(mut self, response: Value) -> TestClient<'a> {
        self.response = Some(response);

        self
    }

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

            // Default to 200 since most tests expect it.
            then = then.status(self.status.unwrap_or(200));
            if let Some(response) = self.response.clone() {
                then.json_body(response);
            }
        })
    }

    fn check<E>(&self, endpoint: E)
    where
        E: Endpoint,
    {
        let result = self.ignore(&endpoint);

        assert!(
            result.is_ok(),
            "expected result to be `Ok`, got `Err`:\n{:#?}",
            result
        );
    }

    fn check_err<E>(
        &self,
        endpoint: E,
        expected_code: u16,
        expected_message: &str,
    ) where
        E: Endpoint,
    {
        let result = self.ignore(&endpoint);

        assert!(
            matches!(
                result,
                Err(Error::Tmdb { code, ref message })
                    if code == expected_code
                        && message == expected_message
            ),
            "expected result to be `Err(Error::Tmdb {{ code: {}, message: \"{}\" }})`, got:\n{:#?}",
            expected_code,
            expected_message,
            result
        );
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

    fn page<'b, E, D>(&'b self, _: &'b E) -> PageIter<'b, Self, E, D>
    where
        E: Pageable,
        D: DeserializeOwned,
    {
        todo!()
    }
}
