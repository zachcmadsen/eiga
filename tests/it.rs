use httpmock::prelude::*;
use httpmock::Mock;
use serde::de::DeserializeOwned;
use ureq::serde_json::{json, Value};

use eiga::{movie, search, Client, Endpoint, Error, PageIter, Pageable, Tmdb};

/// A builder for `TestClient`.
struct TestClientBuilder<'a> {
    method: Option<&'a str>,
    path: Option<&'a str>,
    parameters: Option<&'a [(&'a str, &'a str)]>,
    status: Option<u16>,
    response: Option<Value>,
}

impl<'a> TestClientBuilder<'a> {
    fn build(&self) -> TestClient<'a> {
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
            status: self.status,
            response: self.response.clone(),
        }
    }

    fn method(&mut self, method: &'a str) -> &mut TestClientBuilder<'a> {
        self.method = Some(method);

        self
    }

    fn path(&mut self, path: &'a str) -> &mut TestClientBuilder<'a> {
        self.path = Some(path);

        self
    }

    fn parameters(
        &mut self,
        parameters: &'a [(&'a str, &'a str)],
    ) -> &mut TestClientBuilder<'a> {
        self.parameters = Some(parameters);

        self
    }

    fn status(&mut self, status: u16) -> &mut TestClientBuilder<'a> {
        self.status = Some(status);

        self
    }

    fn response(&mut self, response: Value) -> &mut TestClientBuilder<'a> {
        self.response = Some(response);

        self
    }
}

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
    fn builder() -> TestClientBuilder<'a> {
        TestClientBuilder {
            method: None,
            path: None,
            parameters: None,
            status: None,
            response: None,
        }
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

    fn page<'b, E, D>(&'b self, endpoint: &'b E) -> PageIter<'b, Self, E, D>
    where
        E: Pageable,
        D: DeserializeOwned,
    {
        todo!()
    }
}

fn check<E>(client: TestClient, endpoint: E)
where
    E: Endpoint,
{
    let result = client.ignore(&endpoint);

    assert!(
        result.is_ok(),
        "expected result to be `Ok`, got `Err`:\n{:#?}",
        result
    );
}

fn check_err<E>(
    client: TestClient,
    endpoint: E,
    expected_code: u16,
    expected_message: &str,
) where
    E: Endpoint,
{
    let result = client.ignore(&endpoint);

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

#[test]
fn unprocessable_entity() {
    let expected_code = 422;
    let expected_message = "page must be less than or equal to 500";

    let test_client = TestClient::builder()
        .method("GET")
        .path("search/movie")
        .parameters(&[("query", "Cruel Gun Story"), ("page", "600")])
        .status(expected_code)
        .response(json!({ "errors": [expected_message] }))
        .build();

    let search_movies_endpoint =
        search::Movies::builder("Cruel Gun Story").page(600).build();

    check_err(
        test_client,
        search_movies_endpoint,
        expected_code,
        expected_message,
    );
}

#[test]
fn not_found() {
    let expected_code = 404;
    let expected_message = "The resource you requested could not be found.";

    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/115572")
        .status(expected_code)
        .response(json!({"success":false, "status_code":34, "status_message": expected_message}))
        .build();

    let movie_details_endpoint = movie::Details::builder(115572).build();

    check_err(
        test_client,
        movie_details_endpoint,
        expected_code,
        expected_message,
    );
}

#[test]
fn get_movie_details() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500")
        .parameters(&[("language", "en-US")])
        .build();

    let movie_details_endpoint =
        movie::Details::builder(500).language("en-US").build();

    check(test_client, movie_details_endpoint);
}

#[test]
fn get_movie_alternative_titles() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500/alternative_titles")
        .parameters(&[("country", "US")])
        .build();

    let movie_alternative_titles_endpoint =
        movie::AlternativeTitles::builder(500).country("US").build();

    check(test_client, movie_alternative_titles_endpoint);
}

#[test]
fn get_movie_credits() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("movie/500/credits")
        .parameters(&[("language", "en-US")])
        .build();

    let movie_credits_endpoint =
        movie::Credits::builder(500).language("en-US").build();

    check(test_client, movie_credits_endpoint);
}

#[test]
fn search_movies() {
    let test_client = TestClient::builder()
        .method("GET")
        .path("search/movie")
        .parameters(&[
            ("query", "Samurai Spy"),
            ("language", "en-US"),
            ("include_adult", "false"),
            ("region", "US"),
            ("year", "1965"),
            ("primary_release_year", "1965"),
        ])
        .build();

    let search_movies_endpoint = search::Movies::builder("Samurai Spy")
        .language("en-US")
        .page(1)
        .include_adult(false)
        .region("US")
        .year(1965)
        .primary_release_year(1965)
        .build();

    check(test_client, search_movies_endpoint);
}
